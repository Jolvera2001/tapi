use serde::Serialize;

#[derive(Serialize)]
struct RequestResponse {
    data: String,
    status: u16,
}

#[tauri::command]
async fn make_request(url: &str, method: &str, body: Option<serde_json::Value>) -> Result<RequestResponse, String> {
    let mut request = reqwest::Client::new()
        .request(method.parse().unwrap(), url);

    if let Some(json_body) = body {
        request = request.json(&json_body);
    }

    match request.send().await
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
        .invoke_handler(tauri::generate_handler![make_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
