// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            main_window.listen("window_resized", |event| {
                println!("Window resized: {:?}", event);
            });
            main_window.listen("window_dragged", |event| {
                println!("Window dragged: {:?}", event);
            });
            Ok(())
        })
        .manage(app::playing_info::CachedPlayingInfo::default())
        .manage(app::pinned::WindowPinnedStatus::default())
        .invoke_handler(tauri::generate_handler![app::playing_info::updata_playing_info, app::pinned::toggle_window_pinned])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
