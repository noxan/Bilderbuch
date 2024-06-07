// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{http, AppHandle};

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

fn protocol_preview_handler(
    _app: &AppHandle,
    req: http::Request<Vec<u8>>,
) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    // remove trailing slash from path
    let path = req.uri().path();
    let trim_path = path.trim_start_matches("/");
    let decode_path = percent_encoding::percent_decode(trim_path.as_bytes());
    let filepath = decode_path.decode_utf8_lossy().to_string();

    let file = std::fs::File::open(filepath)?;
    let raw_buf = unsafe { memmap2::Mmap::map(&file)? };
    raw_buf.advise(memmap2::Advice::Random)?;
    let thumbnail = rawtojpg::extract_jpeg(&raw_buf)?;
    let result = thumbnail.to_vec();

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "image/jpg")
        .body(result)?)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![list_directory])
        .register_uri_scheme_protocol("preview", |_app, req| {
            match protocol_preview_handler(_app, req) {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    http::Response::builder()
                        .status(500)
                        .body(Vec::new())
                        .unwrap()
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
