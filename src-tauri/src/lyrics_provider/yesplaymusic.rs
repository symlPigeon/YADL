use std::collections::HashMap;

use crate::metadata::Metadata;

const YESPLAYMUSIC_API: &str = "http://127.0.0.1:27232/api/lyric?id=";

pub type RawLyrics = (Option<String>, Option<String>);

pub async fn fetch_lyrics(metadata: &Metadata) -> RawLyrics {
    let song_id: &str = metadata.song_url.split('/').last().unwrap_or_default();
    let resp = reqwest::get(&format!("{}{}", YESPLAYMUSIC_API, song_id)).await;
    let song_name_lyric = Some(format!("[00:00.000]{}\n[9999:59.990]\n", metadata.title));
    let artist_lyric = Some(format!("[00:00.000]{}\n[9999:59.990]\n", metadata.artist));
    let pure_music_lyric = Some("[00:00.000]纯音乐，请欣赏\n[9999:59.990]\n".to_string());

    if resp.is_err() {
        eprintln!("Failed to fetch lyrics: {:?}", resp.err());
        return (song_name_lyric, artist_lyric);
    }
    let resp = resp.unwrap();
    if !resp.status().is_success() {
        eprintln!("Failed to fetch lyrics: {:?}", resp.status());
        return (song_name_lyric, artist_lyric);
    }
    //get resp body
    let body = resp.json().await;
    if body.is_err() {
        eprintln!("Response body deserialize failed: {:?}", body.err());
        return (song_name_lyric, artist_lyric);
    }
    let body: HashMap<String, serde_json::Value> = body.unwrap();
    if body.get("pureMusic").is_some_and(|flag| flag.as_bool().unwrap_or(false)) {
        return (song_name_lyric, pure_music_lyric);
    }
    let mut lyrics = (None, None);
    if let Some(lrc) = body.get("lrc") {
        if let Some(lyric) = lrc.get("lyric") {
            // replace \\n with \n
            let lyric = lyric.to_string().replace("\\n", "\n");
            // add a fake timestamp to the end
            let lyric = format!("{}\n[9999:59.990]\n", lyric);
            lyrics.0 = Some(lyric.to_string());
        }
    }
    if let Some(tlyric) = body.get("tlyric") {
        if let Some(tlyric) = tlyric.get("lyric") {
            // replace \\n with \n
            let tlyric = tlyric.to_string().replace("\\n", "\n");
            // add a fake timestamp to the end
            let tlyric = format!("{}\n[9999:59.990]\n", tlyric);
            lyrics.1 = Some(tlyric.to_string());
        }
    }
    lyrics
}