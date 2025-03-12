// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

pub mod config;

use chitose;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::string::String;
use std::sync::Mutex;
use tauri::Manager;
use tokio::runtime::Runtime;

use config::config::Config;

struct MkJsonState {
    config: Mutex<Config>,
}

impl MkJsonState {
    fn new() -> Self {
        let config = Config::default();
        MkJsonState {
            config: Mutex::new(config),
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn create_rt() -> Runtime {
    tokio::runtime::Runtime::new().unwrap()
}

#[tauri::command]
fn http_get(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    dbg!(cookie_str);
    let rt = create_rt();
    rt.block_on(async { chitose::http_get(url_str, cookie_str, headers, data_str).await })
}

#[tauri::command]
fn http_post(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    dbg!(cookie_str);
    let rt = create_rt();
    rt.block_on(async { chitose::http_post(url_str, cookie_str, headers, data_str).await })
}

#[tauri::command]
fn http_put(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    dbg!(cookie_str);
    let rt = create_rt();
    rt.block_on(async { chitose::http_put(url_str, cookie_str, headers, data_str).await })
}

#[tauri::command]
fn http_delete(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    dbg!(cookie_str);
    let rt = create_rt();
    rt.block_on(async { chitose::http_delete(url_str, cookie_str, headers, data_str).await })
}

#[tauri::command]
fn load_config(state: tauri::State<'_, MkJsonState>) -> Result<Config, String> {
    let toml_content = fs::read_to_string("config.toml");
    if toml_content.is_err() {
        let message = format!("Error reading config file. {:?}", toml_content.err());
        eprintln!("{}", message);
        return Err(message);
    }

    let config: Config = toml::from_str(&toml_content.unwrap()).unwrap();
    println!("{:?}", config);

    *state.config.lock().unwrap() = config.clone();
    Ok(config)
}

#[tauri::command]
fn save_config(state: tauri::State<'_, MkJsonState>, config: Config) {
    dbg!(&config);

    let mut file = File::create("config.toml").unwrap();
    let toml_content = toml::to_string(&config).unwrap();
    write!(file, "{}", toml_content).unwrap();
    file.flush().unwrap();

    *state.config.lock().unwrap() = config.clone();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            load_config,
            save_config,
            http_get,
            http_post,
            http_put,
            http_delete,
        ])
        .manage(MkJsonState::new())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = window.set_title("MkJson");

            // open dev tools only debug build
            #[cfg(debug_assertions)]
            window.open_devtools();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
