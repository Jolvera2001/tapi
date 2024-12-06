use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RequestResponse {
    pub data: String,
    pub status: u16,
}

#[tauri::command]
pub async fn make_request(url: &str, method: &str, body: Option<serde_json::Value>) -> Result<RequestResponse, String> {
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