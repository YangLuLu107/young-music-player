use std::{fs, io};
use serde::{Serialize, Deserialize};
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Serialize, Deserialize)]
struct FileOrDir {
    key: u64,
    label: String,
    path: String,
    is_directory: bool,
    children: Vec<FileOrDir>,
}
/// 检查文件是否为音乐文件
fn is_music_file<P: AsRef<Path>>(path: P) -> bool {
    match path.as_ref().extension().and_then(|ext| ext.to_str()) {
        Some("mp3") | Some("wav") | Some("flac") | Some("ogg") | Some("m4a") | Some("aac") | Some("ape") | Some("midi") | Some("wma") | Some("aiff") | Some("au") => true,
        _ => false,
    }
}
/// 递归地列出目录下的所有文件和子目录，并返回一个包含文件和目录信息的向量
fn list_files_and_directories_internal<P: AsRef<Path>>(dir: P, counter: &mut u64) -> io::Result<FileOrDir> {
    let path = dir.as_ref();
    let label = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let is_directory = path.is_dir();
    let mut children = Vec::new();
    // let current_key = if let Some(parent) = parent_key {
    //     format!("{}{:03}", parent, *counter)
    // } else {
    //     format!("{:03}", *counter)
    // };

    if is_directory {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() && !is_music_file(&entry_path) {
                continue; // 如果不是音乐文件则跳过
            }
            children.push(list_files_and_directories_internal(entry_path, counter)?);
        }
    }

    *counter += 1; // 增加计数器

    Ok(FileOrDir {
        key: *counter,
        label,
        path: path.to_string_lossy().into_owned(),
        is_directory,
        children,
    })
}

#[tauri::command]
fn list_files_and_directories(dir_path: &str, initial_counter: u64) -> Result<FileOrDir, String> {
    let mut counter = initial_counter;
    list_files_and_directories_internal(dir_path, &mut counter).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![list_files_and_directories])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
