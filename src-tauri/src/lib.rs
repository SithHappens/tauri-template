use serde::Serialize;
use serde_with_macros::skip_serializing_none;
use ts_rs::TS;


// Example to showcase how the Typescript bindings are generated from this struct,
// so this is the single source of truth.
#[skip_serializing_none]
#[derive(Serialize, TS, Debug)]
#[ts(export)]
pub struct Task {
    pub id: String,
    pub ctime: String,
    pub project_id: String,
    pub done: bool,
    pub title: String,
    pub description: Option<String>,
}


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
