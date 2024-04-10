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
            let sec_part = timestamp.split(':').last().unwrap_or_default();
            if sec_part.is_empty() {
                continue;
            }
            let sec_part: Vec<&str> = sec_part.split('.').collect();
            if sec_part.len() != 2 {
                continue;
            }
            let second = sec_part[0].parse::<i64>().unwrap_or_default();
            let millisec = sec_part[1].parse::<i64>().unwrap_or_default();
            (second, millisec)
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