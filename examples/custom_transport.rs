use yars::{
    http::{HttpRequest, HttpResponse},
    protocol::HttpProtocol,
    transport::{Transport, TransportResult},
    Result, YarsServer,
};

struct TestTransport;

impl Transport for TestTransport {
    type Connection = i32;

    async fn bind(&mut self, local_addr: impl tokio::net::ToSocketAddrs) -> TransportResult<()> {
        todo!()
    }

    async fn accept(&self) -> TransportResult<Self::Connection> {
        todo!()
    }

    async fn read(&self, conn: &mut Self::Connection) -> TransportResult<Vec<u8>> {
        todo!()
    }

    async fn write(&self, conn: &mut Self::Connection, response: &[u8]) -> TransportResult<()> {
        todo!()
    }

    async fn shutdown_conn(&self, conn: Self::Connection) -> TransportResult<()> {
        todo!()
    }
}

async fn index(_req: HttpRequest) -> Result<impl Into<HttpResponse>> {
    Ok(HttpResponse::Ok().header("a", "b").json(r#"{"abc": 123}"#))
}

#[tokio::main]
async fn main() -> yars::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    YarsServer::new(TestTransport, HttpProtocol)
        .get("/", index)
        .listen("127.0.0.1:8000")
        .await
}
