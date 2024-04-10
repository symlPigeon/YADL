#[derive(Debug, Default)]
pub struct Metadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub art_url: String,
    pub duration: i64,
    pub song_url: String,
}

impl Metadata {
    pub fn new(
        title: String,
        artist: String,
        album: String,
        art_url: String,
        duration: i64,
        song_url: String,
    ) -> Self {
        Self {
            title,
            artist,
            album,
            art_url,
            duration,
            song_url,
        }
    }
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.artist == other.artist
            && self.album == other.album
            && self.art_url == other.art_url
            && self.duration == other.duration
            && self.song_url == other.song_url
    }
}
