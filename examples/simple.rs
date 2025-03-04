use log::LevelFilter;
use yars::{
    http::{HttpRequest, HttpResponse},
    YarsServer,
};

fn index(_req: HttpRequest) -> impl Into<HttpResponse> {
    HttpResponse::Ok().header("a", "b").json(r#"{"abc": 123}"#)
}

fn okay(_req: HttpRequest) -> impl Into<HttpResponse> {
    "ok"
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
            HttpResponse::Ok().html(include_str!("clickme.html"))
        })
        .listen("127.0.0.1:8000")
        .await
}
