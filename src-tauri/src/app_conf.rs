use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Serialize, Deserialize, Default)]
pub struct AppConf {
    pub app_window_last_x: u32,
    pub app_window_last_y: u32,
    pub app_window_last_width: u32,
    pub app_window_last_height: u32,
    pub app_last_theme: String,
}

#[derive(Default)]
pub struct CachedAppConf {
    pub app_conf: tokio::sync::Mutex<AppConf>,
}

pub fn get_window_default_loc(app: &tauri::window::Window) -> (u32, u32, u32, u32) {
    let prim_monitor = app.primary_monitor();
    // get primiary monitor height and width
    if let Ok(Some(monitor)) = prim_monitor {
        let monitor_size = monitor.size();
        let monitor_width = monitor_size.width;
        let monitor_height = monitor_size.height;
        let quarter_width = monitor_width / 4;
        let quarter_height = monitor_height / 4;
        
        let monitor_loc = monitor.position();
        (
            quarter_width + monitor_loc.x as u32,
            quarter_height * 3 + monitor_loc.y as u32,
            monitor_width / 2,
            monitor_height / 8,
        )
    } else {
        (0, 0, 800, 600)
    }
}

pub fn load_app_conf(app: &tauri::window::Window) -> Option<AppConf> {
    // get config file path
    let config_file_path = tauri::api::path::config_dir();
    config_file_path.as_ref()?;
    let config_file_path = config_file_path.unwrap();
    let config_file_path = config_file_path.join("yadl");
    if !config_file_path.exists() {
        std::fs::create_dir_all(&config_file_path).unwrap();
    }
    let config_file_path = config_file_path.join("app_conf.json");

    if config_file_path.exists() {
        let app_conf_str = std::fs::read_to_string(&config_file_path).unwrap();
        let app_conf = serde_json::from_str(&app_conf_str);
        if let Ok(app_conf) = app_conf {
            return Some(app_conf);
        }
    }

    let (x, y, width, height) = get_window_default_loc(app);
    let app_conf = AppConf {
        app_window_last_x: x,
        app_window_last_y: y,
        app_window_last_width: width,
        app_window_last_height: height,
        app_last_theme: String::from("light_blue"),
    };
    let app_conf_str = serde_json::to_string(&app_conf).unwrap();
    std::fs::write(&config_file_path, app_conf_str).unwrap();
    Some(app_conf)
}

pub async fn refresh_window_state(app: &tauri::AppHandle) -> Result<(), ()> {
    let app_conf_state = app.state::<CachedAppConf>();
    let mut app_conf = app_conf_state.app_conf.lock().await;
    let window = app.get_window("main").unwrap();
    let position = window.inner_position().unwrap();
    let size = window.inner_size().unwrap();
    let (app_window_last_x, app_window_last_y) = (position.x, position.y);
    let (app_window_last_width, app_window_last_height) = (size.width, size.height);
    app_conf.app_window_last_x = app_window_last_x as u32;
    app_conf.app_window_last_y = app_window_last_y as u32;
    app_conf.app_window_last_width = app_window_last_width;
    app_conf.app_window_last_height = app_window_last_height;
    Ok(())
}

pub async fn save_app_conf(app: &tauri::AppHandle) -> Result<(), ()> {
    let app_conf_state = app.state::<CachedAppConf>();
    let app_conf = app_conf_state.app_conf.lock().await;
    let config_file_path = tauri::api::path::config_dir().unwrap();
    if !config_file_path.exists() {
        std::fs::create_dir_all(&config_file_path).unwrap();
    }
    let config_file_path = config_file_path.join("yadl").join("app_conf.json");
    let app_conf_str = serde_json::to_string(&*app_conf).unwrap();
    std::fs::write(config_file_path, app_conf_str).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn change_window_theme(app: tauri::AppHandle, theme: String) -> Result<(), ()> {
    let app_conf_state = app.state::<CachedAppConf>();
    let mut app_conf = app_conf_state.app_conf.lock().await;
    app_conf.app_last_theme = theme;
    drop(app_conf); // THE LOCK MUST BE DROPPED HERE!
                    // OTHERWISE A DEADLOCK WILL OCCUR!
    let _ = save_app_conf(&app).await;
    Ok(())
}

#[tauri::command]
pub async fn get_init_theme(app: tauri::AppHandle) -> String {
    let app_conf_state = app.state::<CachedAppConf>();
    let app_conf = app_conf_state.app_conf.lock().await;
    app_conf.app_last_theme.clone()
}