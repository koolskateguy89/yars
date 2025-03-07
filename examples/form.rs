use log::LevelFilter;
use yars::{
    http::{HttpRequest, HttpResponse},
    Result, YarsServer,
};

fn index(_req: HttpRequest) -> Result<impl Into<HttpResponse>> {
    Ok(HttpResponse::Ok().html(include_str!("form/index.html")))
}

fn submit(_req: HttpRequest) -> Result<impl Into<HttpResponse>> {
    Ok(HttpResponse::Ok().json(r#"{ "abc": 123 }"#))
}

#[tokio::main]
async fn main() -> yars::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let server = YarsServer::default_server()
        .get("/", index)
        .post("/abc", submit);

    server.listen("127.0.0.1:8001").await
}
