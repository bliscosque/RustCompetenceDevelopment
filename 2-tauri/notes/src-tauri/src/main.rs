// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn convert_markdown(text: &str) -> String {
    let html = markdown::to_html(text);
    html
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![convert_markdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
