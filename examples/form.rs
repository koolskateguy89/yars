use log::LevelFilter;
use yars::{HttpRequest, HttpResponse, HttpServer};

fn index(_req: HttpRequest) -> impl Into<HttpResponse> {
    HttpResponse::Ok().html(include_str!("form/index.html"))
}

fn submit(_req: HttpRequest) -> impl Into<HttpResponse> {
    HttpResponse::Ok().json(r#"{ "abc": 123 }"#)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let server = HttpServer::default().get("/", index).post("/abc", submit);

    server.listen("127.0.0.1:8001").await
}
