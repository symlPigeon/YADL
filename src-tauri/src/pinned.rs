use tauri::Manager;

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
