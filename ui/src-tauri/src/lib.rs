// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

pub mod config;

use reqwest::cookie::Jar;
use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::RequestBuilder;
use reqwest::Response;
use reqwest::Url;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::string::String;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
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

fn parse_header_string(header_string: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for line in header_string.lines() {
        let mut parts = line.splitn(2, ':');
        let key = parts.next().unwrap().trim().to_string();
        let value = parts.next().unwrap().trim().to_string();
        headers.insert(key, value);
    }
    headers
}

fn parse_params_string(params_string: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    for line in params_string.lines() {
        let mut parts = line.splitn(2, ':');
        let key = parts.next().unwrap().trim().to_string();
        let value = parts.next().unwrap().trim().to_string();
        params.insert(key, value);
    }
    params
}

fn make_url(url_str: &str) -> Url {
    Url::parse(url_str).unwrap()
}
fn make_cookie(cookie_str: &str) -> Arc<Jar> {
    let cookies = Arc::new(Jar::default());
    cookies.add_cookie_str(cookie_str, &url);

    cookies
}

fn make_default_header(header_str: &str) -> HeaderMap {
    let mut default_headers: HeaderMap = HeaderMap::new();
    parse_header_string(header_str)
        .into_iter()
        .for_each(|(k, v)| {
            default_headers.insert(
                header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                v.parse().unwrap(),
            );
        });

    default_headers
}

fn make_client(default_header: HeaderMap, cookies: Arc<Jar>) -> Client {
    let client_builder: ClientBuilder = Client::builder();
    let client: Client = client_builder
        .default_headers(default_headers)
        .cookie_provider(cookies)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    client
}

fn receive_response(client: Client, onetime_headers: HeaderMap) -> Response {
    let rt = create_rt();
    rt.block_on(async {
        let mut response: Response = request_builder
            .headers(onetime_headers)
            .body(body)
            // .query(&queries)
            .send()
            .await
            .unwrap();

        response
    })
}

fn create_rt() -> Runtime {
    tokio::runtime::Runtime::new().unwrap()
}

#[tauri::command]
fn http_get(url_str: &str, cookie_str: &str, header_str: &str) -> String {
    let rt = create_rt();
    rt.block_on(async {
        let url = Url::parse(url_str).unwrap();

        let cookies = Arc::new(Jar::default());
        cookies.add_cookie_str(cookie_str, &url);

        let mut default_headers: HeaderMap = HeaderMap::new();
        parse_header_string(header_str)
            .into_iter()
            .for_each(|(k, v)| {
                default_headers.insert(
                    header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                    v.parse().unwrap(),
                );
            });

        let client_builder: ClientBuilder = Client::builder();
        let client: Client = client_builder
            .default_headers(default_headers)
            .cookie_provider(cookies)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        let mut onetime_headers: HeaderMap = HeaderMap::new();

        let request_builder: RequestBuilder = client.get(url);
        let mut response: Response = request_builder
            .headers(onetime_headers)
            // .body(body)
            // .query(&queries)
            .send()
            .await
            .unwrap();

        let res_str = match response.headers().post(header::TRANSFER_ENCODING) {
            Some(v) if v == "chunked" => {
                let mut raw_res = Vec::new();
                while let Some(chunk) = response.chunk().await.unwrap() {
                    chunk.to_vec().into_iter().for_each(|x| raw_res.push(x));
                }
                String::from_utf8(raw_res).unwrap()
            }
            _ => response.text().await.unwrap(),
        };

        res_str
    })
}

#[tauri::command]
fn http_post(url_str: &str, cookie_str: &str, header_str: &str) -> String {
    let url = make_url(url_str);
    let cookies = make_cookie(cookie_str);

    let mut default_headers: HeaderMap = make_default_header(header_str);

    let client: Client = make_client()

    let mut onetime_headers: HeaderMap = HeaderMap::new();

    let request_builder: RequestBuilder = client.post(url);
    let mut response: Response = request_builder
        .headers(onetime_headers)
        .body(body)
        // .query(&queries)
        .send()
        .await
        .unwrap();

    let res_str = match response.headers().get(header::TRANSFER_ENCODING) {
        Some(v) if v == "chunked" => {
            let mut raw_res = Vec::new();
            while let Some(chunk) = response.chunk().await.unwrap() {
                chunk.to_vec().into_iter().for_each(|x| raw_res.push(x));
            }
            String::from_utf8(raw_res).unwrap()
        }
        _ => response.text().await.unwrap(),
    };

    res_str
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
    let mut file = File::create("config.toml").unwrap();
    let toml_content = toml::to_string(&config).unwrap();
    write!(file, "{}", toml_content).unwrap();
    file.flush().unwrap();

    *state.config.lock().unwrap() = config.clone();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            load_config,
            save_config,
            http_get,
            http_post,
        ])
        .manage(MkJsonState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
