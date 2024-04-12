use crate::lyrics_provider;

#[tauri::command]
pub async fn toggle_pause_resume() {
    let provider = lyrics_provider::dbus_searcher::get_current_audio_provider().await;
    let provider = match provider.first() {
        Some(provider) => provider,
        None => return,
    };
    lyrics_provider::dbus_searcher::player_toggle_pause(provider).await;
}