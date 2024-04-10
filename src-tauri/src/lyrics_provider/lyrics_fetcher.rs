use crate::metadata::Metadata;

use super::{lyrics_parser, yesplaymusic};

pub async fn get_lyrics(provider: &str, metadata: &Metadata) -> (Vec<(i64, String)>, Vec<(i64, String)>) {
    match provider {
        "yesplaymusic" => {
            let (lyrics, tlyrics) = lyrics_parser::parse_lyrics(yesplaymusic::fetch_lyrics(metadata).await);
            (lyrics, tlyrics)
        },
        _ => {
            let line1_lyrics = format!("[00:00.000]{}\n[99:99.999]\n", metadata.title);
            let line2_lyrics = format!("[00:00.000]{}\n[99:99.999]\n", metadata.artist);
            let (lyrics, tlyrics) = lyrics_parser::parse_lyrics((Some(line1_lyrics), Some(line2_lyrics)));
            (lyrics, tlyrics)
        }
    }
}