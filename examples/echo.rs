use yars::{
    http::{HttpRequest, HttpResponse},
    Result, YarsServer,
};

fn echo(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().text(req.body.unwrap()))
}

fn echo_uri(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().text(req.uri))
}

#[tokio::main]
async fn main() -> yars::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    YarsServer::default_server()
        .post("/echo", echo)
        .default_handler(echo_uri)
        .listen("127.0.0.1:8000")
        .await
}
