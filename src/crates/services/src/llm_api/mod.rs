use alta_s_api::send_to_altas;
use groq_api::send_to_groq;
use tracing::*;
use reqwest::Client;
use shared::{configuration::CONFIG, types::AiRecognizeMethod};

mod alta_s_api;
mod groq_api;

// todo: покрыть все ошибки, а не те которые мне по кайфу щас
#[derive(Debug)]
pub enum AiRequestError {
    GroqApiKey,
    GroqRequest,
    AltaSUrl,
    AltaSRequest,
}

// todo: rewrite to result
/// Returns `None` if token unspecified or ошибка случилась
pub async fn send_request(req: String) -> Result<String, AiRequestError> {
    match &CONFIG.ai.recognize_method {
        AiRecognizeMethod::Groq => {
            if &CONFIG.ai.groq_token == "NOT" {
                warn!("GROQ is curently uses for Ai features, but token was not specified.");
                Err(AiRequestError::GroqRequest)
            } else {
                send_to_groq(req).await
            }
        }
        AiRecognizeMethod::AltaS => send_to_altas(req).await,
        AiRecognizeMethod::None => Err(AiRequestError::GroqRequest), // nothing for recognize, so just return command
    }
}

async fn request(
    client: Client,
    url: &str,
    api_key: String,
    body: serde_json::Value,
) -> reqwest::Response {
    client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .unwrap()
}
