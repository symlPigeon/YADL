use crate::{lyrics_provider, metadata::Metadata};

pub struct PlayingInfo {
    pub metadata: Metadata,
    pub provider: String,
    pub cached_lyrics: Vec<(i64, String)>,
    pub cached_tlyrics: Vec<(i64, String)>,
    pub position: i64,
    pub last_update_time: std::time::Instant,
}

impl Default for PlayingInfo {
    fn default() -> Self {
        Self {
            metadata: Metadata::default(),
            provider: String::default(),
            cached_lyrics: Vec::new(),
            cached_tlyrics: Vec::new(),
            position: 0,
            last_update_time: std::time::Instant::now(),
        }
    }
}

#[derive(Default)]
pub struct CachedPlayingInfo {
    pub playing_info: tokio::sync::Mutex<PlayingInfo>,
}

#[tauri::command]
pub async fn updata_playing_info(
    playing_state: tauri::State<'_, CachedPlayingInfo>,
) -> Result<String, String> {
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
        None => {
            return Ok(format!(
                "<p data-tauri-drag-region>Enjoy Yourself with {}!</p>",
                provider
            ))
        }
    };
    // If the song is the same, just update the position & lyrics, there is no need to update metadata
    // otherwise we have to update the metadata and the lyrics (costly)
    let mut inner_playing_info = playing_state.playing_info.lock().await;
    if inner_playing_info.metadata != metadata || &inner_playing_info.provider != provider {
        // now we are playing different song, so we have to update the metadata and the lyrics
        inner_playing_info.metadata = metadata;
        inner_playing_info.provider = provider.to_string();
        inner_playing_info.cached_lyrics.clear();
        inner_playing_info.last_update_time = std::time::Instant::now();
        // Update lyrics
        let (lyrics, tlyrics) =
            lyrics_provider::lyrics_fetcher::get_lyrics(provider, &inner_playing_info.metadata)
                .await;
        println!("Lyrics: {:#?}", lyrics);
        println!("TLyrics: {:#?}", tlyrics);
        inner_playing_info.cached_lyrics = lyrics;
        inner_playing_info.cached_tlyrics = tlyrics;
    }
    // Update position
    let mut position = lyrics_provider::dbus_searcher::get_current_playing_position(provider)
        .await
        .unwrap_or(0);
    if position == inner_playing_info.position
        && lyrics_provider::dbus_searcher::get_playback_status(provider)
            .await
            .unwrap_or("Playing".to_string())
            != "Paused"
    {
        // is playing and dbus not updated
        position += inner_playing_info.last_update_time.elapsed().as_micros() as i64;
    } else {
        inner_playing_info.position = position;
        inner_playing_info.last_update_time = std::time::Instant::now();
    }

    // Update lyrics according to the position
    let current_lyric_upper = inner_playing_info
        .cached_lyrics
        .windows(2)
        .find_map(|lyric| {
            let (prev_lyric, post_lyric) = (lyric[0].clone(), lyric[1].clone());
            if prev_lyric.0 <= position / 1000 && post_lyric.0 > position / 1000 {
                Some(prev_lyric.1)
            } else {
                None
            }
        })
        .unwrap_or(inner_playing_info.metadata.title.clone());
    let current_lyric_lower = inner_playing_info
        .cached_tlyrics
        .windows(2)
        .find_map(|lyric| {
            let (prev_lyric, post_lyric) = (lyric[0].clone(), lyric[1].clone());
            if prev_lyric.0 <= position / 1000 && post_lyric.0 > position / 1000 {
                Some(prev_lyric.1)
            } else {
                None
            }
        })
        .unwrap_or(inner_playing_info.metadata.artist.clone());

    let lyrics = format!(
        "<p data-tauri-drag-region>{}</p><p data-tauri-drag-region>{}</p>",
        current_lyric_upper, current_lyric_lower
    );
    Ok(lyrics)
}
