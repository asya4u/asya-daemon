use macros::Stringify;

use shared::configuration::CONFIG;
use tracing::*;

use crate::scenarios::*;

/// Usecases are the main business logic of the application.
///
/// This usecases module contains all the possible actions that the user can perform from client.
#[derive(Debug, Stringify, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Usecases {
    /// Turns off current track.
    /// # Examples
    ///  - Asya, turns off the music, please.
    ///  - Shut up music
    TurnOffMusic,

    /// Turns on current track.
    /// # Examples
    ///  - Asya, turn the music back on.
    ///  - Resume the song
    TurnOnMusic,

    /// Returns currently playing track.
    ///
    /// # Examples
    ///  - What song is playing right now?
    ///  - What's the name of the current track?
    GetMusicStatus,

    /// Play next track.
    ///
    /// # Examples
    ///  - Next song, please.
    ///  - Skip to the next track.
    PlayNextTrack,

    /// Play previous track.
    ///
    /// # Examples
    ///  - Play the previous song.
    ///  - Go back to the last track.
    PlayPrevTrack,

    /// Turns off the computer.
    Shutdown,

    /// Reboot computer.
    Reboot,

    /// Open specified app.
    OpenApp(String),

    /// Basic system monitoring.
    StartBasicSystemMonitoring,

    /// If no other options are suitable, then this is a simple request from a language model.
    ///
    /// # Examples
    ///  - Can you help me with that?
    ///  - Please provide an answer.
    Answer,
}

impl Usecases {
    pub fn stringify_all() -> String {
        let strings = [Usecases::stringify_one()];
        let iter = strings.iter().map(|el| el.to_string() + "\n\n");
        String::from_iter(iter)
    }
    pub async fn execute(self, userinput: String) {
        let command = self;
        debug!("Dispatching command: {:?}", command);
        match command {
            Usecases::TurnOffMusic | Usecases::TurnOnMusic => {
                music_control::play_or_resume_music(userinput).await;
            }
            Usecases::GetMusicStatus => {
                music_control::get_music_status(userinput).await;
            }
            Usecases::PlayNextTrack => music_control::play_next_track(userinput).await,
            Usecases::PlayPrevTrack => music_control::play_previous_track(userinput).await,
            Usecases::StartBasicSystemMonitoring => {
                system_monitoring::start_basic_monitoring(userinput).await
            }
            Usecases::OpenApp(app) => open_app::open(app).await,
            Usecases::Answer => geranal_answer::answer(userinput).await,
            Usecases::Shutdown => {
                do_dang(pc_mgmt::shutdown::shutdown).await;
            }
            Usecases::Reboot => {
                do_dang(pc_mgmt::shutdown::reboot).await;
            }
        }
    }
}

async fn do_dang(h: impl std::ops::AsyncFnOnce()) {
    if CONFIG.usecases.test_dangerous_features {
        info!("Execute dangerous feature.");
        h().await
    } else {
        info!("Dangerous features doesn't execute.");
    }
}
