use tauri::Manager;

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_make_request_success() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(200).set_body_string("test response"))
            .mount(&mock_server)
            .await;

        let result = make_request(
            &format!("{}/test", mock_server.uri()),
            "GET",
            None
        ).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.data, "test response");
    }
}


// #[tokio::test]
// async fn test_make_request() {
//     let app = tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![make_request])
//         .build(tauri::test::mock_context())
//         .unwrap();
// }