# YARS - Yet Another Rust Server

## Project Overview

YARS is a lightweight server framework written from scratch in Rust. It focuses on **raw data transport, protocol parsing, and request routing**, serving as a hands-on exploration into **low-level server design**. This project is intended for learning purposes and does not implement the full feature set of mature frameworks like Actix-Web or Axum.


## TODO

- [ ] custom transport example
- [ ] echo example (need to handle http body parsing first)
- [ ] integration tests with `/tests` dir https://doc.rust-lang.org/book/ch11-03-test-organization.html
- [ ] mini-redis example
- [ ] web app example (simple, just a few pages with a form or smthn)
- [ ] ? UDP transport
- [ ] more protocol implementations (e.g. HTTP/2, Bencode)

### Example Usage

See the [`examples`](examples) directory for more comprehensive examples.

```rust
//! examples/hello_world.rs

use yars::{
    http::{HttpRequest, HttpResponse, RequestMethod},
    protocol::HttpProtocol,
    transport::TcpTransport,
    YarsServer,
};

// Handlers can return any Result<Into<HttpResponse>, Into<Box<dyn std::error::Error>>>
fn hello(_req: HttpRequest) -> anyhow::Result<HttpResponse> {
    Ok(HttpResponse::Ok().text("Hello, World!"))
}

fn not_found(_req: HttpRequest) -> yars::Result<HttpResponse> {
    Ok(HttpResponse::NotFound().text("Not Found"))
}

#[tokio::main]
async fn main() -> yars::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    YarsServer::new(TcpTransport::default(), HttpProtocol)
        .route(("/", RequestMethod::GET), hello)
        // note http protocol exposes shorthand helpers
        .get("/hello", hello)
        // can define a default handler for all unhandled routes
        .default_handler(not_found)
        .listen("127.0.0.1:8000")
        .await
}
```

## Known Issues/Limitations

- Max TCP request size of 1024 bytes
- Probably doesn't actually fully implement HTTP 1/1.1 spec
- No support for HTTP [`Trailer` headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Trailer)
- No ergonomic way to return error responses
  - No ergonomic way to handle handler errors
