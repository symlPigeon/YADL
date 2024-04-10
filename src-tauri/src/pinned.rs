use tauri::{Manager, PhysicalSize, Size};

#[derive(Default)]
pub struct WindowPinnedStatus {
    is_pinned: tokio::sync::Mutex<bool>,
}

#[tauri::command]
pub async fn toggle_window_pinned(app: tauri::AppHandle, state: tauri::State<'_, WindowPinnedStatus>) -> Result<String, String> {
    let mut is_pinned = state.is_pinned.lock().await;
    *is_pinned = !*is_pinned;
    println!("Window pinned status: {}", if *is_pinned { "pinned" } else { "unpinned" });
    // app.emit_to("main", "toggle_pin_status", *is_pinned).unwrap();
    let main_window = app.get_window("main").unwrap();
    let _ = main_window.set_ignore_cursor_events(*is_pinned);

    Ok(is_pinned.to_string())
}

#[tauri::command]
pub async fn sync_window_position(app: tauri::AppHandle) -> Result<(), String> {
    let main_window = app.get_window("main").unwrap();
    let mut main_window_pos = main_window.inner_position().unwrap();
    // add a offset to the position
    main_window_pos.x += 15;
    main_window_pos.y += 5;
    let pin_window = app.get_window("pin").unwrap();
    pin_window.set_position(main_window_pos).unwrap();
    // move pin_window to top
    pin_window.set_focus().unwrap();
    Ok(())
}

#[tauri::command]
pub async fn reset_pin_window_size(app: tauri::AppHandle) -> Result<(), String> {
    let pin_window = app.get_window("pin").unwrap();
    let size = Size::Physical(PhysicalSize {
        width: 20,
        height: 20,
    });
    pin_window.set_size(size).unwrap();
    Ok(())
}