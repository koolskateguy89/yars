use http_server::{HttpRequest, HttpResponse, HttpServer};
use log::LevelFilter;

fn test_i_guess(req: HttpRequest) -> HttpResponse {
    dbg!(req);
    HttpResponse::Ok()
}

fn main() -> std::io::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let server = HttpServer::default()
        .get("/", |_req: HttpRequest| {
            HttpResponse::Ok().json(r#"{"abc": 123}"#)
        })
        .get("/test", |_req: HttpRequest| {
            HttpResponse::Ok().html(include_str!("../erm.html"))
        })
        .get("/test2", |_req| "abc")
        .get("/abc", test_i_guess);

    server.listen("127.0.0.1:8000")
}
