mod heart;
mod machine;
mod request;
pub mod support_system;

use machine::State;

fn main() {
    heart::init();

    match iced::run(State::update, State::view) {
        Ok(_) => (),
        Err(_) => println!("-----"),
    }
}
