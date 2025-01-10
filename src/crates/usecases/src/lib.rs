use crate::usecases::Usecases;
use serde::Serialize;
use services::llm_api;
use shared::event_system;
use shared::plugin_system::ReadableRequest;
use std::sync::Arc;
use tokio::task;
use tracing::*;

pub mod scenarios;
pub mod shared_workers;
mod tools;
pub mod usecases;

fn process_response(llm_response: &str) -> Result<Usecases, Box<dyn std::error::Error>> {
    let llm_response = llm_response.replace("`json", "");
    // removes first '{' and last '}'
    let llm_response = remove_braces(llm_response.as_str());
    let llm_response = llm_response.replace("`", "");
    println!("{}", &llm_response);
    let usecase = serde_json::from_str::<Usecases>(&llm_response.clone())?;
    Ok(usecase)
}

fn remove_braces(input: &str) -> String {
    let mut result = input.to_string();

    let left_count = result.matches('{').count();

    if left_count < 2 {
        if let Some(pos) = result.find('{') {
            result.remove(pos);
        }

        if let Some(pos) = result.rfind('}') {
            result.remove(pos);
        }
    }

    result
}

pub async fn subscribe_for_plugins() {
    event_system::subscribe_once({
        move |event: Arc<ReadableRequest>| {
            task::spawn(async move {
                dispatch_by_user_message(event.0.to_string()).await;
            })
        }
    })
    .await
}

pub async fn dispatch_by_user_message(message: String) {
    let schema = schemars::schema_for!(Usecases);
    let stringified_schema = serde_json::to_string_pretty(&schema)
        .unwrap()
        .replace("\n", "");
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    let c_req = format!(
        "
            Determine whether the following user input: {} is similar to any of the commands below. 
            {}
            If it is similar, respond with a single word: TRUE. 
            If it is not similar, respond with a single word: FALSE.
        ",
        message,
        serde_json::to_string_pretty(&schema).unwrap()
    );

    let c_llm_response = llm_api::send_request(c_req).await;
    let usecase = if c_llm_response.unwrap() == "TRUE" {
        let g_req = format!(
            "
                Generate json representation of command from this user input: {}
                by this json schema fo available commands: {}

                SEND ME ONLY GENERATED JSON

            ",
            message, stringified_schema
        );

        let g_llm_response = llm_api::send_request(g_req).await;

        if g_llm_response.is_err() {
            warn!("Error sending request to LLM: {:?}", g_llm_response.err());
            return;
        }
        let llm_response = g_llm_response.unwrap();
        debug!("LLM RESPONSE: {}", llm_response);
        let usecase = process_response(&llm_response);
        if let Err(err) = usecase {
            warn!("Error parsing response from LLM: {:?}", err);
            return;
        }
        usecase.unwrap()
    } else {
        usecases::Usecases::Answer
    };
    usecase.execute(message).await;
}

// general purpose events

/// General response event. Use it to send responses to the client.
/// How event works see [`shared::event_system`].
#[derive(Debug, parse_display::Display, Serialize)]
#[serde(tag = "asyaResponse")]
#[serde(rename_all = "camelCase")]
pub enum AsyaResponse {
    /// Success response with message from Asya.
    ///
    /// # Arguments
    ///     * `message` - human readable message from Asya, e.g.
    ///         "I've turned off the music. Don't listen this shit anymore."
    ///
    /// # Example
    ///
    /// ```
    /// event_system::publish(AsyaResponse::Ok {
    ///     message: "Hi, Vitaliy! I heard that u like thinkpads? Me too!"
    /// }
    ///
    /// ```
    #[display("{message}")]
    Ok { message: String },

    /// Represents error.
    ///
    /// Although this `AsyaResponse`, this invariant represents message that will be shouldn't
    /// displayed to user.
    #[display("{message}")]
    Err { message: String },
}
