use features::lexicon::get_lexicon;
use features::{
    llm_api,
    services::commands::music::{self, MediaPlayingStatus},
};
use shared::{traits::Beautify, utils::llm_utils};
use teloxide::types::Message;

pub async fn dispatch_music_command(command: String, msg: &Message) -> String {
    match command.as_str() {
        "pause" | "resume" => {
            let music_status = music::get_status();
            music::play_pause();
            match music_status {
                MediaPlayingStatus::Stopped => get_lexicon("music_stopped").to_string(),
                MediaPlayingStatus::Paused(_) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/resume");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(get_lexicon("music_resume").to_string())
                }
                MediaPlayingStatus::Playing(_) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/pause");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(get_lexicon("music_pause").to_string())
                }
                MediaPlayingStatus::Unknown => get_lexicon("music_stopped").to_string(),
            }
        }
        "status" => {
            let music_status = music::get_status();
            match music_status {
                MediaPlayingStatus::Stopped => get_lexicon("music_stopped").to_string(),
                MediaPlayingStatus::Paused(status) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/status");
                    let formatted_prompt = prompt
                        .replace("{status}", format!("{:?}", status).as_str())
                        .replace("{message}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(status.beautiful_out())
                    // todo: beautify
                    // music output
                }
                MediaPlayingStatus::Playing(status) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/status");
                    let formatted_prompt = prompt
                        .replace("{status}", format!("{:?}", status).as_str())
                        .replace("{message}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(status.beautiful_out())
                    // todo: beautify
                    // music output
                }
                MediaPlayingStatus::Unknown => get_lexicon("music_stopped").to_string(),
            }
        }
        _ => get_lexicon("error").to_string(),
    }
}
