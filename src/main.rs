use http_server::{HttpRequest, HttpResponse, HttpServer};

fn _test_i_guess(req: HttpRequest) -> HttpResponse {
    dbg!(req);
    HttpResponse::new()
}

fn main() -> std::io::Result<()> {
    let server = HttpServer::default()
        .get("/", |req: HttpRequest| {
            dbg!(req);
            HttpResponse::new()
        })
        .post("/test", |_req| HttpResponse::new())
        .get("/test2", |_req| "abc");

    server.listen("127.0.0.1:8000")
}
