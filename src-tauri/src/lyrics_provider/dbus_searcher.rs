use std::collections::HashMap;

use zbus::{proxy, zvariant::Str, Connection};

use crate::metadata::Metadata;

#[proxy(
    interface = "org.freedesktop.DBus",
    default_service = "org.freedesktop.DBus",
    default_path = "/org/freedesktop/DBus"
)]
trait DBusCommands {
    async fn ListNames(&self) -> zbus::Result<Vec<String>>;
}

pub async fn get_current_audio_provider() -> Vec<String> {
    let connection = Connection::session().await.unwrap();
    let proxy = DBusCommandsProxy::new(&connection).await.unwrap();
    let names = match proxy.ListNames().await {
        Ok(names) => names,
        Err(e) => {
            eprintln!("{:#?}", e);
            return Vec::new();
        }
    };

    let provider_names = names
        .iter()
        .filter(|name| name.starts_with("org.mpris.MediaPlayer2."))
        .map(|name| name.replace("org.mpris.MediaPlayer2.", ""))
        .collect::<Vec<String>>();

    provider_names
}

pub async fn get_playing_metadata(provider: &str) -> Option<Metadata> {
    let connection = Connection::session().await.unwrap();
    let resp = connection
        .call_method(
            Some(format!("org.mpris.MediaPlayer2.{}", provider)),
            "/org/mpris/MediaPlayer2",
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &("org.mpris.MediaPlayer2.Player", "Metadata"),
        )
        .await;
    let resp = match resp {
        Ok(resp) => resp.body(),
        Err(e) => {
            eprintln!("{:#?}", e);
            return None;
        }
    };
    let metadata: zbus::zvariant::Value = match resp.deserialize() {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("{:#?}", e);
            return None;
        }
    };
    let metadata: HashMap<zbus::zvariant::Str, zbus::zvariant::Value> =
        match HashMap::try_from(metadata) {
            Ok(metadata) => metadata,
            Err(e) => {
                eprintln!("{:#?}", e);
                return None;
            }
        };

    let mut song_metadata = Metadata::new("".into(), "".into(), "".into(), "".into(), 0, "".into());

    for (key, value) in metadata.iter() {
        match key as &str {
            "mpris:artUrl" => {
                let art_url = Str::try_from(value).unwrap();
                song_metadata.art_url = art_url.into();
            }
            "mpris:length" => {
                let length = i64::try_from(value).unwrap();
                song_metadata.duration = length;
            }
            "xesam:title" => {
                let title = Str::try_from(value).unwrap();
                song_metadata.title = title.into();
            }
            "xesam:album" => {
                let album = Str::try_from(value).unwrap();
                song_metadata.album = album.into();
            }
            "xesam:artist" => {
                let artist_field = zbus::zvariant::Array::try_from(value).unwrap();
                let artist = Str::try_from(&artist_field[0]).unwrap();
                song_metadata.artist = artist.into();
            }
            "xesam:url" => {
                let song_url = Str::try_from(value).unwrap();
                song_metadata.song_url = song_url.into();
            }
            _ => {}
        }
    }

    Some(song_metadata)
}

pub async fn get_current_playing_position(provider: &str) -> Option<i64> {
    let connection = Connection::session().await.unwrap();
    let resp = connection
        .call_method(
            Some(format!("org.mpris.MediaPlayer2.{}", provider)),
            "/org/mpris/MediaPlayer2",
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &("org.mpris.MediaPlayer2.Player", "Position"),
        )
        .await;
    let resp = match resp {
        Ok(resp) => resp.body(),
        Err(e) => {
            eprintln!("{:#?}", e);
            return None;
        }
    };
    let position: zbus::zvariant::Value = resp.deserialize().unwrap();
    let position = i64::try_from(position).unwrap();
    Some(position)
}

pub async fn get_playback_status(provider: &str) -> Option<String> {
    let connection = Connection::session().await.unwrap();
    let resp = connection
        .call_method(
            Some(format!("org.mpris.MediaPlayer2.{}", provider)),
            "/org/mpris/MediaPlayer2",
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &("org.mpris.MediaPlayer2.Player", "PlaybackStatus"),
        )
        .await;
    let resp = match resp {
        Ok(resp) => resp.body(),
        Err(e) => {
            eprintln!("{:#?}", e);
            return None;
        }
    };
    let status: zbus::zvariant::Value = resp.deserialize().unwrap();
    let status = Str::try_from(status).unwrap();
    Some(status.into())
}

pub async fn player_toggle_pause(provider: &str) {
    let connection = Connection::session().await.unwrap();
    // We just ignore that :(
    // If anything wrong, blame the up stream.
    let _ = connection
        .call_method(
            Some(format!("org.mpris.MediaPlayer2.{}", provider)),
            "/org/mpris/MediaPlayer2",
            Some("org.mpris.MediaPlayer2.Player"),
            "PlayPause",
            &(),
        )
        .await;
}
