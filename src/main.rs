mod constants;
mod response;
mod server;

fn main() -> std::io::Result<()> {
    let server = server::HttpServer::default();
    server.listen("127.0.0.1:8000")
}
