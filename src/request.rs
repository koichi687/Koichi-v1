use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    User,
    Model,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSaving {
    pub role: Role,
    pub text: String,
    pub time: String,
}

pub async fn req(history: Vec<MessageSaving>) -> Result<String, String> {
    dotenvy::dotenv().ok();

    //can change this
    let key =
        std::env::var("API_KEY").map_err(|_| "API_KEY tidak ditemukan di .env".to_string())?;

    let config_json = std::fs::read_to_string("config.json").unwrap_or_default();

    let config: serde_json::Value = serde_json::from_str(&config_json).unwrap_or_default();

    //endpoint gemini
    let endp = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-flash-lite:generateContent?key={}",
        key
    );

    let content: Vec<serde_json::Value> = history
        .iter()
        .map(|msg| {
            let role = match msg.role {
                Role::User => "user".to_string(),
                Role::Model => "model".to_string(),
            };

            json!({
                "role": role,
                "parts": [
                    {"text": msg.text}
                ]
            })
        })
        .collect();

    let personality_on_json = if let Some(arr) = config["personality"].as_array() {
        arr.iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<&str>>()
            .join("\n")
    } else {
        "".to_string()
    };

    //sambungan dir support

    let data = crate::support_system::directory::scan_read("storage/docs");

    //req to api gemini first
    let full = format!("{} \n\n [dokumen]: \n{}", personality_on_json, data);
    //json req
    let body = json!({
        "system_instruction": {
            "parts": [
                {"text": full}
            ]
        },

        "generation_config": {
            "max_output_tokens": 150,
            "temperature": 0.6
        },

        "contents": content,
          "safetySettings": [
        {
            "category": "HARM_CATEGORY_HARASSMENT",
            "threshold": "BLOCK_NONE"
        },
        {
            "category": "HARM_CATEGORY_HATE_SPEECH",
            "threshold": "BLOCK_NONE"
        },
        {
            "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
            "threshold": "BLOCK_NONE"
        },
        {
            "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
            "threshold": "BLOCK_NONE"
        }
    ]
    });

    //rakit
    let client = reqwest::Client::new();
    let resp: serde_json::Value = client
        .post(&endp)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Koneksi gagal: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Gagal membaca JSON: {}", e))?;

    match resp["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        Some(text) => Ok(text.to_string()),
        None => match resp["error"]["message"].as_str() {
            Some(err) => Err(format!("Error API: {}", err)),
            None => Err("Format respons tidak dikenali".to_string()),
        },
    }
}
