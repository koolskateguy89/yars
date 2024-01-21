use http_server::{HttpRequest, HttpResponse, HttpServer};
use log::LevelFilter;

fn test_i_guess(req: HttpRequest) -> HttpResponse {
    dbg!(req);
    HttpResponse::new(200)
}

fn main() -> std::io::Result<()> {
    // pretty_env_logger::formatted_builder()
    //     .filter_level(LevelFilter::Debug)
    //     .init();

    let server = HttpServer::default()
        .get("/", |_req: HttpRequest| {
            HttpResponse::new(200).json(r#"{"abc": 123}"#)
        })
        .post("/test", |_req| HttpResponse::new(404))
        .get("/test2", |_req| "abc")
        .get("/abc", test_i_guess);

    server.listen("127.0.0.1:8000")
}
