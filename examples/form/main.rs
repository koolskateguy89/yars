use yars::{
    http::{HttpRequest, HttpResponse},
    Result, YarsServer,
};

async fn index(_req: HttpRequest) -> Result<impl Into<HttpResponse>> {
    Ok(HttpResponse::Ok().html(include_str!("index.html")))
}

async fn submit(_req: HttpRequest) -> Result<impl Into<HttpResponse>> {
    Ok(HttpResponse::Ok().json(r#"{ "abc": 123 }"#))
}

#[tokio::main]
async fn main() -> yars::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    YarsServer::default_server()
        .get("/", index)
        .post("/submit", submit)
        .listen("127.0.0.1:8001")
        .await
}
