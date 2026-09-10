use std::time::Instant;

use crate::request::{MessageSaving, Role};
use chrono::Local;
use iced::widget::{button, column, container, row, scrollable, space, text, text_input};
use iced::{Element, Fill, Task};

pub struct State {
    pub history: Vec<MessageSaving>,
    pub input: String,
    pub waiting: bool,
    pesan_terakhir: Option<Instant>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChange(String),
    Send,
    ResponAi(Result<String, String>),
}

impl Default for State {
    fn default() -> Self {
        Self {
            history: crate::heart::load(),
            input: String::new(),
            waiting: false,
            pesan_terakhir: None,
        }
    }
}

impl State {
    pub fn update(&mut self, chat: Message) -> Task<Message> {
        match chat {
            Message::InputChange(text) => {
                self.input = text;
                Task::none()
            }
            Message::Send => {
                if !self.input.trim().is_empty() && !self.waiting {
                    let promt = self.input.trim().to_string();
                    self.input.clear();
                    self.waiting = true;

                    //history, baut bahan llm

                    self.history.push(MessageSaving {
                        role: Role::User,
                        text: promt,
                        time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
                    crate::heart::save_history(&self.history);

                    let history_send = self.history.clone();

                    Task::perform(crate::request::req(history_send), Message::ResponAi)
                } else {
                    Task::none()
                }
            }
            Message::ResponAi(hasil) => {
                self.waiting = false;
                match hasil {
                    Ok(jawaban) => {
                        self.history.push(MessageSaving {
                            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            role: Role::Model,
                            text: jawaban,
                        });
                    }
                    Err(_) => {
                        //     self.history.push(MessageSaving {
                        //         role: Role::Model,
                        //         text: format!("eror: ". err)
                        //     });
                    }
                }
                crate::heart::save_history(&self.history);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let judul = text("k-V1").size(15);

        let send_content: Element<'_, Message> = if self.waiting {
            text(".......").into()
        } else {
            iced::widget::image(iced::widget::image::Handle::from_bytes(
                &include_bytes!("assets/send.png")[..],
            ))
            .width(24)
            .height(24)
            .into()
        };

        let interaction_chat = row![
            text_input("type..", &self.input)
                .on_input(Message::InputChange)
                .on_submit(Message::Send)
                .padding(10),
            button(send_content).on_press(Message::Send),
        ]
        .spacing(10);

        let chat_list = column(self.history.iter().map(|msg| match msg.role {
            Role::User => {
                let buble = container(
                    column![text("admin").size(11), text(&msg.text).size(15),].spacing(4),
                )
                .padding(10)
                .style(container::bordered_box);
                row![space::horizontal(), buble].into()
            }
            Role::Model => {
                let bubble = container(
                    column![text("koichi").size(11), text(&msg.text).size(15),].spacing(4),
                )
                .padding(10)
                .style(container::bordered_box);
                row![bubble, space::horizontal()].into()
            }
        }))
        .spacing(10);

        let scroll = scrollable(chat_list).height(Fill);

        let interaction = column![scroll, interaction_chat,].spacing(10);

        let border_chat = container(interaction)
            .width(Fill)
            .height(Fill)
            .padding(10)
            .style(container::bordered_box);

        let interface = column![judul, border_chat,]
            .spacing(20)
            .padding(20)
            .width(Fill)
            .height(Fill);

        interface.into()
    }
}
