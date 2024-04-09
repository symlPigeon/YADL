// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use app::{lyrics_provider::{self, dbus_searcher::get_current_playing_position}, metadata::Metadata};

#[derive(Default)]
struct PlayingInfo {
    metadata: Metadata,
    provider: String,
    cached_lyrics: HashMap<i64, String>,
    position: i64,
}

#[derive(Default)]
struct CachedPlayingInfo {
    playing_info: tokio::sync::Mutex<PlayingInfo>
}

#[tauri::command]
async fn updata_playing_info(playing_state: tauri::State<'_, CachedPlayingInfo>) -> Result<String, String> {
    // get mpris provider now
    let provider = lyrics_provider::dbus_searcher::get_current_audio_provider().await;
    // In case no provider is found
    let provider = match provider.first() {
        Some(provider) => provider,
        None => return Ok("<p data-tauri-drag-region>Enjoy Yourself with Silence!</p>".to_string()),
    };
    // Update metadata
    let metadata = lyrics_provider::dbus_searcher::get_playing_metadata(provider).await;
    let metadata = match metadata {
        Some(metadata) => metadata,
        None => return Ok(format!("<p data-tauri-drag-region>Enjoy Yourself with {}!</p>", provider)),
    };
    // If the song is the same, just update the position & lyrics, there is no need to update metadata
    // otherwise we have to update the metadata and the lyrics (costly)
    let mut inner_playing_info = playing_state.playing_info.lock().await;
    if inner_playing_info.metadata != metadata || &inner_playing_info.provider != provider {
        // now we are playing different song, so we have to update the metadata and the lyrics
        inner_playing_info.metadata = metadata;
        inner_playing_info.provider = provider.to_string();
        inner_playing_info.cached_lyrics.clear();
        // Update lyrics
        // update_lyrics();
    }
    // Update position
    let position = get_current_playing_position(provider).await.unwrap_or(0);
    inner_playing_info.position = position;

    // Update lyrics according to the position
    let current_time_readable = {
        let seconds = position / 1_000_000;
        let milliseconds = (position % 1_000_000) / 1_000;
        let minutes = seconds / 60;
        let seconds = seconds % 60;
        format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
    };
    let lyrics = format!("<p data-tauri-drag-region>{}!</p><p data-tauri-drag-region>Now position {}</p>", inner_playing_info.metadata.title, current_time_readable);
    Ok(lyrics)
}

fn main() {
    tauri::Builder::default()
        .manage(CachedPlayingInfo::default())
        .invoke_handler(tauri::generate_handler![updata_playing_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
