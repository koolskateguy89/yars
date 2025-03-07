use log::LevelFilter;
use yars::{
    http::{HttpRequest, HttpResponse},
    YarsServer,
};

fn index(_req: HttpRequest) -> anyhow::Result<impl Into<HttpResponse>> {
    let _ = "abc".parse::<i32>()?;

    let num = "abc".parse::<i32>()?;
    print!("{}", num);

    Ok(HttpResponse::Ok().header("a", "b").json(r#"{"abc": 123}"#))
}

fn okay(_req: HttpRequest) -> anyhow::Result<impl Into<HttpResponse>> {
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
        .get("/clickme", |_req: HttpRequest| -> anyhow::Result<_> {
            Ok(HttpResponse::Ok().html(include_str!("clickme.html")))
        })
        .listen("127.0.0.1:8000")
        .await
}
