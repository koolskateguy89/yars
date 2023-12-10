use http_server::{HTTPRequest, HTTPResponse, HttpServer};

fn main() -> std::io::Result<()> {
    let server = HttpServer::default()
        .get("/", |req: HTTPRequest| {
            dbg!(req);
            HTTPResponse::new()
        })
        .post("/test", |_req| HTTPResponse::new())
        .get("/test2", |_req| "abc");

    server.listen("127.0.0.1:8000")
}
