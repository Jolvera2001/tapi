use fantoccini::{Client, ClientBuilder, Locator as By};
use serde_json::{json, Map};

// testing out how fantoccini works
// #[tokio::test]
// async fn test_basic_chrome_connection() -> Result<(), Box<dyn std::error::Error>> {
//     // start new chrome driver session
//     let client = ClientBuilder::native()
//         .connect("http://localhost:9515")
//         .await?;

//     client.goto("https://www.google.com").await?;

//     let title = client.title().await?;
//     println!("Page title: {}", title);

//     client.close().await?;
//     Ok(())
// }

#[tokio::test]
async fn test_request_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut capabilities = Map::new();
    let chrome_options = json!({
        "args": ["--headless", "--no-sandbox", "--disable-dev-shm-usage"]
    });
    capabilities.insert("goog:chromeOptions".to_string(), chrome_options);

    let client = ClientBuilder::native()
        .capabilities(capabilities)
        .connect("http://localhost:4444/wd/hub")
        .await?;

    // Navigate to your local dev server
    client.goto("http://localhost:1420").await?;

    let request_input = client.find(By::Css("[data-testid='request-input'")).await?;
    request_input.send_keys("https://api.example.com").await?;

    client.close().await?;
    Ok(())
}