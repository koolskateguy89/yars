# YARS - Yet Another Rust Server

YARS is a lightweight asynchronous server framework written from scratch in Rust. It focuses on raw data transport, protocol parsing, and request routing, and serving as a hands-on exploration into low-level server design.

This project is intended for learning purposes and does not (will not) implement the full feature set of mature frameworks like Actix-Web or Axum.

Built upon the [`tokio`](https://tokio.rs/) runtime.

## Overview

The server is a cake (everything is a case), you can think of it as having 3 layers:

- transport layer
- protocol layer (may be renamed in future)
- request handler

The [transport](src/transport.rs) layer is responsible for...

The [protocol](src/protocol.rs) layer is responsible for providing strongly-typed request and response objects.
It is also responsible for deserialising bytes from the transport layer into strongly-typed request objects.
Then serialising the response object into bytes for the transporr layer.
smthn about routing...

The request handlers are written by you, the user!
You can define a default handler for if no route is matched.

## TODO

- [x] echo example (need to handle http body parsing first)
- [x] async handlers
- [ ] custom transport example
- [ ] mini-redis example
- [ ] web app example (simple, just a few pages with a form or smthn)
- [ ] ? UDP transport
- [ ] more protocol implementations (e.g. HTTP/2, [Bencode](https://en.wikipedia.org/wiki/Bencode))

## Example Usage

> See the [`examples`](examples) directory for more comprehensive examples.

```rust
//! examples/hello_world.rs

use yars::{
    http::{HttpRequest, HttpResponse, RequestMethod},
    protocol::HttpProtocol,
    transport::TcpTransport,
    YarsServer,
};

// Handlers can return any Result<Into<HttpResponse>, Into<Box<dyn std::error::Error>>>
async fn hello(_req: HttpRequest) -> anyhow::Result<HttpResponse> {
    Ok(HttpResponse::Ok().text("Hello, World!"))
}

async fn not_found(_req: HttpRequest) -> yars::Result<HttpResponse> {
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

## Observability

- Uses [tracing](https://docs.rs/tracing/latest/tracing/) for structured logging

## Known Issues/Limitations

- Max TCP request size of 1024 bytes
- Probably doesn't actually fully implement HTTP 1/1.1 spec
- No support for HTTP [`Trailer` headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Trailer)
- No ergonomic way to return error responses
  - No ergonomic way to handle handler errors
