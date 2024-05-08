use tauri::Manager;

use crate::app_conf::{save_app_conf, CachedAppConf};

#[tauri::command]
pub async fn start_custom_settings(app: tauri::AppHandle) -> Result<(), String> {
    // first, ensure the window is not already open
    let window = app.get_window("CustomSettings");
    if let Some(window) = window {
        window.set_focus().unwrap();
        return Ok(());
    }
    // create new window
    let window = tauri::WindowBuilder::new(
        &app,
        "CustomSettings",
        tauri::WindowUrl::App("theme_setting.html".into()),
    )
    .build()
    .unwrap();
    window
        .set_size(tauri::LogicalSize::new(400.0, 400.0))
        .unwrap();
    window.set_resizable(false).unwrap(); // resize this window makes it ugly :(
    Ok(())
}

#[tauri::command]
pub fn close_theme_setting(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_window("CustomSettings");
    if window.is_none() {
        return Ok(());
    }
    let window = window.unwrap();
    window.close().unwrap();
    Ok(())
}

#[tauri::command]
pub async fn save_theme_setting(app: tauri::AppHandle, text_color: String, shadow_color: String) -> Result<(), String> {
    let app_conf = app.state::<CachedAppConf>();
    let mut app_conf = app_conf.app_conf.lock().await;
    app_conf.custom_text_color = Some(text_color.clone());
    app_conf.custom_shadow_color = Some(shadow_color.clone());
    drop(app_conf);
    let _ = save_app_conf(&app).await;

    // change theme
    let main_window = app.get_window("main").unwrap();
    println!("Emit modify_custom_settings event, with payload: ({}, {})", text_color.clone(), shadow_color.clone());
    let _ = main_window.emit("modify_custom_settings", (text_color, shadow_color));
    // close setting window
    let _ = close_theme_setting(app);

    Ok(())
}
#[tauri::command]
pub async fn init_custom_theme(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_window("main").unwrap();
    let app_conf = app.state::<CachedAppConf>();
    let app_conf = app_conf.app_conf.lock().await;
    let text_color = app_conf.custom_text_color.clone();
    let shadow_color = app_conf.custom_shadow_color.clone();
    drop(app_conf);
    let _ = window.emit("modify_custom_settings", (text_color.unwrap_or("#55a4ff".to_string()), shadow_color.unwrap_or("#fc0".to_string())));
    Ok(())
}