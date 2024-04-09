// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .manage(app::playing_info::CachedPlayingInfo::default())
        .manage(app::pinned::WindowPinnedStatus::default())
        .invoke_handler(tauri::generate_handler![app::playing_info::updata_playing_info, app::pinned::toggle_window_pinned])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
