// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn background_update_position(app: tauri::AppHandle) -> Result<(), String> {
    std::thread::spawn(move || {
        let main_window = app.get_window("main").unwrap();
        let mut cached_position = main_window.inner_position().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
            let current_position = main_window.inner_position().unwrap();
            if cached_position != current_position {
                // println!("Position changed: {:?}", current_position);
                let _ = main_window.emit("update_position", current_position);
            }
            cached_position = current_position;
        }
    });
    Ok(())
}

#[tauri::command]
fn exit_app(app: tauri::AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(app::playing_info::CachedPlayingInfo::default())
        .manage(app::pinned::WindowPinnedStatus::default())
        .invoke_handler(tauri::generate_handler![
            app::playing_info::updata_playing_info,
            app::pinned::toggle_window_pinned,
            app::pinned::sync_window_position,
            background_update_position,
            app::pinned::reset_pin_window_size,
            exit_app,
            app::right_menu::toggle_pause_resume,
            app::pinned::reset_pin_window_focus
        ])
        .plugin(tauri_plugin_context_menu::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
