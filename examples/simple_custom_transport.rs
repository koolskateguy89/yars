use log::LevelFilter;
use yars::{
    protocol::HttpProtocol,
    transport::{Transport, TransportResult},
    YarsServer,
};
use yars::{HttpRequest, HttpResponse};

fn index(_req: HttpRequest) -> impl Into<HttpResponse> {
    HttpResponse::Ok().header("a", "b").json(r#"{"abc": 123}"#)
}

fn okay(_req: HttpRequest) -> impl Into<HttpResponse> {
    "ok"
}

struct TestTransport;

impl Transport for TestTransport {
    type Connection = i32;

    async fn bind(
        &mut self,
        addr: impl tokio::net::ToSocketAddrs,
    ) -> TransportResult<std::net::SocketAddr> {
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

    async fn close(&self, conn: Self::Connection) -> TransportResult<()> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> yars::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Debug)
        .init();

    YarsServer::new(TestTransport, HttpProtocol)
        .get("/", index)
        .get("/ok", okay)
        .get("/clickme", |_req: HttpRequest| {
            HttpResponse::Ok().html(include_str!("clickme.html"))
        })
        .listen("127.0.0.1:8000")
        .await
}
