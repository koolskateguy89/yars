use std::time::Duration;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use yars::{
    http::{HttpRequest, HttpResponse},
    protocol::HttpProtocol,
    transport::TcpTransport,
    YarsServer,
};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

impl From<User> for HttpResponse {
    fn from(value: User) -> Self {
        let user_json = serde_json::to_string(&value).unwrap();
        HttpResponse::Ok().json(user_json)
    }
}

fn json(_req: HttpRequest) -> Result<impl Into<HttpResponse>> {
    Ok(User {
        name: "John".to_string(),
        age: 30,
    })
}

fn text(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().header("a", "b").text("Hello there"))
}

fn test_server() -> YarsServer<TcpTransport, HttpProtocol> {
    YarsServer::default_server()
        .get("/json", json)
        .get("/text", text)
}

#[tokio::test(flavor = "multi_thread")]
async fn tcp_http_server() -> Result<()> {
    let url = "localhost:8000";

    let server_future_handle = tokio::spawn(test_server().listen(url));
    // Wait for server to start
    tokio::time::sleep(Duration::from_millis(1_000)).await;

    // Text response body type
    let text_response = reqwest::get(format!("http://{url}/text")).await?;
    // Check content type
    let text_headers = text_response.headers();
    assert_eq!(text_headers.get("content-type").unwrap(), "text/plain");
    // Check custom header
    assert_eq!(text_headers.get("a").unwrap(), "b");
    // Check body
    let text_body = text_response.text().await?;
    assert_eq!(text_body, "Hello there");

    // Json response body type
    let json_response = reqwest::get(format!("http://{url}/json")).await?;
    let json_headers = json_response.headers();
    // Check content type
    assert_eq!(
        json_headers.get("content-type").unwrap(),
        "application/json"
    );
    // Check body
    let user: User = json_response.json().await?;
    assert_eq!(user.name, "John");
    assert_eq!(user.age, 30);

    // Stop server
    server_future_handle.abort();

    Ok(())
}
