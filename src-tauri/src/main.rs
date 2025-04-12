// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::Read;
use std::str;

#[tauri::command]
fn decompress_gz(path: String) -> Result<String, String> {
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let mut decoder = ZlibDecoder::new(file);
    let mut buffer = Vec::new();
    decoder
        .read_to_end(&mut buffer)
        .map_err(|e| e.to_string())?;

    match str::from_utf8(&buffer) {
        Ok(s) => Ok(s.to_string()), // Successfully decoded to string
        Err(e) => Err(format!("Failed to decode to UTF-8: {}", e)), // Handle UTF-8 error
    }
}

#[tauri::command]
fn compress_gz(content: String, path: String) -> Result<String, String> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::fs::File;
    use std::io::Write;

    // Create the file to write the compressed data
    let file = File::create(&path).map_err(|e| e.to_string())?;

    let mut encoder = ZlibEncoder::new(file, Compression::default());
    encoder
        .write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;
    encoder.finish().map_err(|e| e.to_string())?;

    Ok("Compression successful".to_string())
}

#[tauri::command]
async fn fetch_url(url: String) -> Result<Vec<u8>, String> {
    match reqwest::get(&url).await {
        Ok(resp) => match resp.bytes().await {
            Ok(bytes) => Ok(bytes.to_vec()),
            Err(e) => Err(format!("Failed to read bytes: {}", e)),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            decompress_gz,
            compress_gz,
            fetch_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
