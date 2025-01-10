use crate::tools::PromptBuilder;
use crate::AsyaResponse;
use services::{
    lexicon::Lexicon,
    services::commands::music::{self, MediaPlayingStatus},
};
use shared::{event_system, traits::Beautify};

// for future use

// #[derive(Debug)]
// pub enum MusicControlEvent {
//     MusicPaused { song_name: String },
//     MusicResumed { song_name: String },
// }

/// Plays or resumes music if already playing.
///
/// # Events
///     * [`AsyaResponse::Ok`] - if music was paused or resumed.
pub async fn play_or_resume_music(executed_command: String) {
    let music_status = music::get_status();
    music::play_pause();
    match music_status {
        MediaPlayingStatus::Stopped => (),
        MediaPlayingStatus::Paused(_) => {
            let res = PromptBuilder::new()
                .set_path("/telegram/music/resume")
                .set_variable("{command}", executed_command.as_str())
                .set_fallback_phrase(Lexicon::MusicResume)
                .get_result()
                .await;

            event_system::publish(AsyaResponse::Ok {
                message: res.to_string(),
            })
            .await;
        }
        MediaPlayingStatus::Playing(_) => {
            let res = PromptBuilder::new()
                .set_path("/telegram/music/pause")
                .set_variable("{command}", executed_command.as_str())
                .set_fallback_phrase(Lexicon::MusicPause)
                .get_result()
                .await;

            event_system::publish(AsyaResponse::Ok {
                message: res.to_string(),
            })
            .await;
        }
        MediaPlayingStatus::Unknown => (),
    };
}

/// Gets the current status of the music player.
///
/// Events:
///     * [`AsyaResponse::Ok`] - message will be contain the current status of the music player.
pub async fn get_music_status(userinput: String) {
    let music_status = music::get_status();
    match music_status {
        // MediaPlayingStatus::Stopped => Lexicon::MusicStopped.describe().to_string(),
        MediaPlayingStatus::Stopped => (),
        MediaPlayingStatus::Paused(status) => {
            publish_music_status(status, &userinput).await;
        }
        MediaPlayingStatus::Playing(status) => {
            publish_music_status(status, &userinput).await;
        }
        // MediaPlayingStatus::Unknown => Lexicon::MusicStopped.describe().to_string(),
        MediaPlayingStatus::Unknown => (),
    };
}

async fn publish_music_status(status: music::TrackInfo, userinput: &str) {
    let res = PromptBuilder::new()
        .set_path("/telegram/music/status")
        .set_variable("{status}", status.beautiful_out().as_str())
        .set_variable("{message}", userinput)
        .set_fallback_phrase(Lexicon::ExecuteSuccess)
        .get_result()
        .await;

    event_system::publish(AsyaResponse::Ok {
        message: res.to_string(),
    })
    .await;
}

/// Plays the next track in the playlist.
/// # Events
///     * [`AsyaResponse::Ok`] - message will contain the result of the operation.
pub async fn play_next_track(_: String) {
    music::play_next();

    let res = PromptBuilder::new()
        .set_path("/telegram/music/next")
        .set_fallback_phrase(Lexicon::ExecuteSuccess)
        .get_result()
        .await;

    event_system::publish(AsyaResponse::Ok {
        message: res.to_string(),
    })
    .await;
}

/// Plays the next track in the playlist.
/// # Events
///     * [`AsyaResponse::Ok`] - message will contain the result of the operation.
pub(crate) async fn play_previous_track(_: String) {
    music::play_prev();

    let res = PromptBuilder::new()
        .set_path("/telegram/music/prev")
        .set_fallback_phrase(Lexicon::ExecuteSuccess)
        .get_result()
        .await;

    event_system::publish(AsyaResponse::Ok {
        message: res.to_string(),
    })
    .await;
}
