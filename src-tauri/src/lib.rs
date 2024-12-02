use serde::{Serialize};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct RequestResponse {
    data: String,
    status: u16,
}

#[tauri::command]
async fn make_request(url: &str, method: &str) -> Result<RequestResponse, String> {
    match reqwest::Client::new()
        .request(method.parse().unwrap(), url)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status().as_u16();
            let data = response.text().await.unwrap();
            Ok(RequestResponse {data, status})
        },
        Err(e) => Err(e.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, make_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
