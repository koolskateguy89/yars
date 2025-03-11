use yars::{
    http::{HttpRequest, HttpResponse, RequestMethod},
    protocol::HttpProtocol,
    transport::TcpTransport,
    YarsServer,
};

// Handlers can return any Result<Into<HttpResponse>, Into<Box<dyn std::error::Error>>>
fn hello(_req: HttpRequest) -> anyhow::Result<HttpResponse> {
    Ok(HttpResponse::Ok().text("Hello, World!"))
}

fn not_found(_req: HttpRequest) -> yars::Result<HttpResponse> {
    Ok(HttpResponse::NotFound().text("Not Found"))
}

#[tokio::main]
async fn main() -> yars::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    YarsServer::new(TcpTransport::default(), HttpProtocol)
        .route(("/", RequestMethod::GET), hello)
        // note http protocol exposes shorthand helpers
        .get("/hello", hello)
        // can define a default handler for all unhandled routes
        .default_handler(not_found)
        .listen("127.0.0.1:8000")
        .await
}
