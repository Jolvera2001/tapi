// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn make_request(url: &str) -> Result<String, String> {
    match reqwest::get(url).await {
        Ok(response) => Ok(response.text().await.unwrap()),
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
