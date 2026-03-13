mod commands;
mod config;
mod services;
mod types;
mod utils;

use services::HttpClient;

/// 전체 Tauri Command를 한 곳에서 등록하는 오케스트레이터.
/// Electron의 `setupIpcHandlers()` (ipcHandlers.ts) 역할을 대체합니다.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(HttpClient::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::auth::auth_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
