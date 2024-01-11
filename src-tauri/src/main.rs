// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_files(path: &str) -> Vec<String> {
    let mut file_paths = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for entry in paths {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            file_paths.push(entry.path().to_string_lossy().to_string())
        }
    }
    return file_paths;
}
