use std::fs;
use std::process::Command;
use std::path::Path;
use std::env;

#[tauri::command]
fn read_config() -> Result<String, String> {
    fs::read_to_string("config.txt").map_err(|e| e.to_string())
}

fn get_mp4_folder() -> String {
    #[cfg(target_os = "windows")]
    let home = env::var("USERPROFILE").unwrap();
    #[cfg(not(target_os = "windows"))]
    let home = env::var("HOME").unwrap();
    Path::new(&home).join("Video Hub Videos").to_string_lossy().to_string()
}

const MP4_EXTENSION: &str = ".mp4";

#[tauri::command]
fn get_mp4_folder_cmd() -> String {
    get_mp4_folder()
}

#[tauri::command]
fn download_video(url: String, path: String) -> Result<(), String> {
    let response = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let mp4_folder = get_mp4_folder();
    let downloaded_path = format!("{}/{}{}", mp4_folder, path, MP4_EXTENSION);

    fs::create_dir_all(Path::new(&mp4_folder)).map_err(|e| e.to_string())?;

    let bytes = response.bytes().map_err(|e| e.to_string())?;
    fs::write(&downloaded_path, bytes).map_err(|e| e.to_string())
}

#[tauri::command]
fn play_video(path: String) -> Result<(), String> {
    let full_path = format!("{}/{}{}", get_mp4_folder(), path, MP4_EXTENSION);
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &full_path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        Command::new("mpv")
            .arg(&full_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn get_video_list() -> Result<Vec<String>, String> {
    let mp4_folder = get_mp4_folder();
    fs::create_dir_all(Path::new(&mp4_folder)).map_err(|e| e.to_string())?;
    let mut files = Vec::new();
    for entry in fs::read_dir(&mp4_folder).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("mp4") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                files.push(name.to_string());
            }
        }
    }
    files.sort();
    Ok(files)
}

#[tauri::command]
fn play_all_videos() -> Result<(), String> {
    let mp4_folder = get_mp4_folder();
    fs::create_dir_all(Path::new(&mp4_folder)).map_err(|e| e.to_string())?;
    let mut paths = Vec::new();
    for entry in fs::read_dir(&mp4_folder).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("mp4") {
            paths.push(path.to_string_lossy().to_string());
        }
    }
    paths.sort();
    if paths.is_empty() {
        return Err("No videos found".to_string());
    }
    #[cfg(target_os = "windows")]
    {
        // Try vlc first for sequential play
        match Command::new("vlc").args(&paths).spawn() {
            Ok(_) => Ok(()),
            Err(_) => {
                // Fallback to start each
                for p in paths {
                    Command::new("cmd")
                        .args(["/C", "start", "", &p])
                        .spawn()
                        .map_err(|e| e.to_string())?;
                }
                Ok(())
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        Command::new("mpv")
            .args(&paths)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_config, download_video, play_video, play_all_videos, get_video_list, get_mp4_folder_cmd])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
