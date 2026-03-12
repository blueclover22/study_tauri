use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn verify_credentials(username: &str, password: &str) -> bool {
    let default_users = vec![
        User {
            username: "admin".to_string(),
            password: "admin123".to_string(),
        },
        User {
            username: "user".to_string(),
            password: "user123".to_string(),
        },
    ];

    default_users
        .iter()
        .any(|user| user.username == username && user.password == password)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, verify_credentials])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
