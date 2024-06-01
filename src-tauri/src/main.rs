// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(serde::Serialize)]
struct Metadata {
    accessed: std::time::SystemTime,
    modified: std::time::SystemTime,
    created: std::time::SystemTime,
}

#[derive(serde::Serialize)]
struct Item {
    name: String,
    path: String,
    metadata: Metadata,
    is_directory: bool,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn list_directory(path: &str) -> Vec<Item> {
    let files = std::fs::read_dir(path).unwrap();
    return files
        .map(|file| {
            let file = file.unwrap();
            let metadata = file.metadata().unwrap();
            return Item {
                name: file.file_name().into_string().unwrap(),
                path: file.path().display().to_string(),
                metadata: Metadata {
                    accessed: metadata.accessed().unwrap(),
                    modified: metadata.modified().unwrap(),
                    created: metadata.created().unwrap(),
                },
                is_directory: file.file_type().unwrap().is_dir(),
            };
        })
        .collect::<Vec<Item>>();
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![list_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
