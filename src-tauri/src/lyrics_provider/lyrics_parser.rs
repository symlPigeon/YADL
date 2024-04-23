use super::yesplaymusic::RawLyrics;

fn parse_lyric_channel(lyric_lines: Vec<&str>) -> Vec<(i64, String)> {
    let mut lyrics_map = Vec::new();
    for lyric_line in lyric_lines.iter() {
        let timestamp = lyric_line.split(']').next().unwrap_or_default().split('[').last().unwrap_or_default();
        if timestamp.is_empty() {
            continue;
        }
        let minute = timestamp.split(':').next().unwrap_or_default();
        if minute.is_empty() {
            continue;
        }
        let minute = minute.parse::<i64>().unwrap_or_default();
        let (second, millisec) = {
            let splitted_timestamp: Vec<&str> = timestamp.split(':').collect();
            // [mm:ss.SS] or [mm:ss:SS]
            match splitted_timestamp.len() {
                2 => {
                    let parts = splitted_timestamp[1].split('.').collect::<Vec<&str>>();
                    if parts.len() != 2 {
                        continue;
                    }
                    let second = parts[0].parse::<i64>().unwrap_or_default();
                    let millisec = parts[1].parse::<i64>().unwrap_or_default();
                    (second, millisec)
                },
                3 => {
                    let second = splitted_timestamp[1].parse::<i64>().unwrap_or_default();
                    let millisec = splitted_timestamp[2].parse::<i64>().unwrap_or_default();
                    (second, millisec)
                },
                _ => continue,
            }
        };
        let timestamp = 1000 * (minute * 60 + second) + millisec;
        // remove timestamp part
        // find first ']'
        let timestamp_end = lyric_line.find(']').unwrap(); // Now this line must have ]
        let lyric_line = &lyric_line[timestamp_end + 1..];
        lyrics_map.push((timestamp, lyric_line.to_string()));
    }
    lyrics_map
}

pub fn parse_lyrics(lyrics: RawLyrics) -> (Vec<(i64, String)>, Vec<(i64, String)>) {
    let (lyric, tlyric) = lyrics;
    // Now we just assume that translation is always synced with the original lyrics
    let lyric = lyric.unwrap();
    let tlyric = tlyric.unwrap_or_default();
    let lyric_lines = lyric.lines().collect::<Vec<&str>>();
    let tlyric_lines = tlyric.lines().collect::<Vec<&str>>();

    let parsed_lyric = parse_lyric_channel(lyric_lines);
    let parsed_tlyric = parse_lyric_channel(tlyric_lines);

    (parsed_lyric, parsed_tlyric)
}