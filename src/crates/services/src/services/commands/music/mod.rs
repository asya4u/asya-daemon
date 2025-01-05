use std::fmt::Display;
use shared::traits::Beautify;

// =======================================
// General data structure TrackInfo
// =======================================
#[derive(Debug, PartialEq, Clone)]
pub struct TrackInfo {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
}

impl Display for TrackInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatting_query = vec![];
        if let Some(artist_value) = &self.artist {
            formatting_query.push(format!("artist: {}", artist_value))
        }
        if let Some(album_value) = &self.album {
            formatting_query.push(format!("album: {}", album_value))
        }
        if let Some(title_value) = &self.title {
            formatting_query.push(format!("title: {}", title_value))
        }
        let res = String::from_iter(formatting_query);
        write!(f, "{}", res)
    }
}

impl Beautify for TrackInfo {
    fn beautiful_out(&self) -> String {
        let res = match self.album.as_ref() {
            None => format!(
                "🎧 <b>Сейчас играет:</b>\n\n{} — {}",
                self.artist.clone().unwrap_or_default(),
                self.title.clone().unwrap_or_default()
            ),
            Some(album) => format!(
                "🎧 <b>Сейчас играет:</b>\n\n{} — {}\nАльбом: {}",
                self.artist.clone().unwrap_or_default(),
                self.title.clone().unwrap_or_default(),
                album
            ),
        };
        res
    }
}

#[derive(Debug)]
pub enum MediaPlayingStatus {
    Playing(TrackInfo),
    Paused(TrackInfo),
    Stopped,
    Unknown,
}

impl Display for MediaPlayingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MediaPlayingStatus::Playing(track_info) | MediaPlayingStatus::Paused(track_info) => {
                write!(f, "{}", track_info)
            }
            _ => write!(f, ""),
        }
    }
}

// =======================================
// Implementation for Linux (MPRIS)
// =======================================
#[cfg(target_family = "unix")]
use mpris::{PlayerFinder, Player};
#[cfg(target_family = "unix")]
use std::error::Error;

#[cfg(target_family = "unix")]
fn get_player() -> Result<Player, Box<dyn Error>> {
    let finder = PlayerFinder::new()?;
    let player = finder.find_active()?;
    Ok(player)
}

#[cfg(target_family = "unix")]
pub fn play_pause() {
    if let Ok(player) = get_player() {
        let _ = player.play_pause();
    }
}

#[cfg(target_family = "unix")]
pub fn get_status() -> MediaPlayingStatus {
    if let Ok(player) = get_player() {
        let metadata = player.get_metadata().ok();
        let track_info = TrackInfo {
            title: metadata.as_ref().and_then(|m| m.title().map(|s| s.to_string())),
            artist: metadata.as_ref().and_then(|m| m.artists().map(|a| a.join(", "))),
            album: metadata.as_ref().and_then(|m| m.album_name().map(|s| s.to_string())),
        };

        match player.get_playback_status().unwrap_or_default() {
            mpris::PlaybackStatus::Playing => MediaPlayingStatus::Playing(track_info),
            mpris::PlaybackStatus::Paused => MediaPlayingStatus::Paused(track_info),
            mpris::PlaybackStatus::Stopped => MediaPlayingStatus::Stopped,
        }
    } else {
        MediaPlayingStatus::Unknown
    }
}

#[cfg(target_family = "unix")]
pub fn play_next() -> Result<(), String> {
    if let Ok(player) = get_player() {
        player.next().map_err(|e| e.to_string())
    } else {
        Err("Player not found".to_string())
    }
}

#[cfg(target_family = "unix")]
pub fn play_prev() -> Result<(), String> {
    if let Ok(player) = get_player() {
        player.previous().map_err(|e| e.to_string())
    } else {
        Err("Player not found".to_string())
    }
}

// =======================================
// Implementation for Windows 
// =======================================
#[cfg(target_family = "windows")]
use winapi::um::winuser::{
    keybd_event, KEYEVENTF_KEYUP, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK,
};

// ======================================
// Start/stop function
// ======================================
#[cfg(target_family = "windows")]
pub fn play_pause() {
    unsafe {
       keybd_event(VK_MEDIA_PLAY_PAUSE as u8, 0, 0, 0);
       keybd_event(VK_MEDIA_PLAY_PAUSE as u8, 0, KEYEVENTF_KEYUP, 0);
    }
}

// ======================================
// Status acquisition function (stub)
// ======================================
#[cfg(target_family = "windows")]
pub fn get_status() -> MediaPlayingStatus {
    let track_info = TrackInfo {
        title: Some("Unknown Title".to_string()),
        artist: Some("Unknown Artist".to_string()),
        album: Some("Unknown Album".to_string()),
    };
    MediaPlayingStatus::Playing(track_info)
}

// ======================================
// Function Next Track
// ======================================
#[cfg(target_family = "windows")]
pub fn play_next() -> Result<(), String>{
    unsafe {
        keybd_event(VK_MEDIA_NEXT_TRACK as u8, 0, 0, 0);
        keybd_event(VK_MEDIA_NEXT_TRACK as u8, 0, KEYEVENTF_KEYUP, 0);
    }
    Ok(())
}

// ======================================
// Function Previous Track
// ======================================
#[cfg(target_family = "windows")]
pub fn play_prev() -> Result<(), String>{
    unsafe {
        keybd_event(VK_MEDIA_PREV_TRACK as u8, 0, 0, 0);
        keybd_event(VK_MEDIA_PREV_TRACK as u8, 0, KEYEVENTF_KEYUP, 0);
    }
    Ok(())
}




