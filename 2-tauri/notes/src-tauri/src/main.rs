// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn convert_markdown(text: &str) -> String {
    let html = markdown::to_html(text);
    html
}

#[tauri::command]
async fn open_editor(handler: tauri::AppHandle, editor_id: &str) -> Result<(), tauri::Error> {
    let editor_window=tauri::WindowBuilder::new(&handler, editor_id, tauri::WindowUrl::App(("editor/".to_string()+editor_id).parse().unwrap()))
        .build()
        .unwrap();
    Ok(())
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![convert_markdown,open_editor])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
