// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_hotkey_capture() -> String {
    format!("start_hotkey_capture!!!")
}

#[tauri::command]
fn stop_hotkey_capture() -> String {
    format!("stop_hotkey_capture!!!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_hotkey_capture, stop_hotkey_capture])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
