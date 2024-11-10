
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
/*
#[derive(Serialize, Deserialize)]
struct FileEntry {
    path: String,
    is_directory: bool,
}
#[tauri::command]
fn get_directory_contents(path: &str) -> Result<Vec<FileEntry>, String> {
    let path = PathBuf::from(path);
    if !path.exists() || !path.is_dir() {
        return Err(format!("路径不存在或不是一个目录: {}", path.display()));
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        let is_directory = metadata.is_dir();
        entries.push(FileEntry {
            path: path.to_string_lossy().to_string(),
            is_directory,
        });
    }

    Ok(entries)
}*/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
