use log::LevelFilter;
use yars::{
    http::{HttpRequest, HttpResponse},
    Result, YarsServer,
};

fn index(_req: HttpRequest) -> Result<impl Into<HttpResponse>> {
    Ok(HttpResponse::Ok().header("a", "b").json(r#"{"abc": 123}"#))
}

fn okay(_req: HttpRequest) -> Result<impl Into<HttpResponse>> {
    Ok("ok")
}

#[tokio::main]
async fn main() -> yars::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Debug)
        .init();

    YarsServer::default_server()
        .get("/", index)
        .get("/ok", okay)
        .get("/clickme", |_req: HttpRequest| {
            Ok(HttpResponse::Ok().html(include_str!("clickme.html")))
        })
        .listen("127.0.0.1:8000")
        .await
}
