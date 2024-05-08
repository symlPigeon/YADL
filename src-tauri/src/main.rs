// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::app_conf::{refresh_window_state, save_app_conf};
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
async fn exit_app(app: tauri::AppHandle) -> Result<(), String> {
    println!("Start EXIT sequence...");
    let _ = refresh_window_state(&app).await;
    println!("Refresh window state finished!");
    let _ = save_app_conf(&app).await;
    println!("Save app_conf finished! Exiting...");
    app.exit(0);
    Ok(())
}

fn setup_window(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_handler = app.handle();
    let window = app_handler.get_window("main");
    if window.is_none() {
        return Ok(());
    }
    let window = window.unwrap();
    let app_conf = app::app_conf::load_app_conf(&window);
    if app_conf.is_none() {
        return Ok(());
    }
    let app_conf = app_conf.unwrap();
    let position = tauri::LogicalPosition::new(app_conf.app_window_last_x as f64, app_conf.app_window_last_y as f64);
    let size = tauri::LogicalSize::new(app_conf.app_window_last_width as f64, app_conf.app_window_last_height as f64);
    let _ = window.set_position(position);
    let _ = window.set_size(size);

    // add app_conf to app state
    app.manage(app::app_conf::CachedAppConf {
        app_conf: tokio::sync::Mutex::new(app_conf)
    });
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(setup_window)
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
            app::pinned::reset_pin_window_focus,
            app::app_conf::change_window_theme,
            app::app_conf::get_init_theme,
            app::custom_theme::start_custom_settings,
            app::custom_theme::close_theme_setting,
            app::custom_theme::save_theme_setting,
            app::custom_theme::init_custom_theme,
        ])
        .plugin(tauri_plugin_context_menu::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
