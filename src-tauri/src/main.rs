// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::http;

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
fn list_directory(path: &str) -> Result<Vec<Item>, String> {
    let files = match std::fs::read_dir(path) {
        Ok(files) => files,
        Err(e) => return Err(e.to_string()),
    };
    return match files
        .map(|file| {
            let file = file?;
            let metadata = file.metadata()?;
            let name = file
                .file_name()
                .into_string()
                .map_err(|_| format!("Invalid file name: {:?}", file.file_name()))?;
            Ok(Item {
                name: name,
                path: file.path().display().to_string(),
                metadata: Metadata {
                    accessed: metadata.accessed()?,
                    modified: metadata.modified()?,
                    created: metadata.created()?,
                },
                is_directory: file.file_type()?.is_dir(),
            })
        })
        .collect::<Result<Vec<Item>, Box<dyn std::error::Error>>>()
    {
        Ok(items) => Ok(items),
        Err(e) => Err(e.to_string()),
    };
}

fn protocol_preview_handler(
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
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![list_directory])
        .register_uri_scheme_protocol("preview", |_ctx, req| match protocol_preview_handler(req) {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Error: {}", e);
                http::Response::builder()
                    .status(500)
                    .body(Vec::new())
                    .unwrap()
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
