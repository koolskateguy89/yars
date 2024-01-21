#![allow(dead_code)]

use log::{debug, info};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use crate::constants;
use crate::request::{HttpRequest, RequestMethod};
use crate::response::HttpResponse;

type Handler = dyn Fn(HttpRequest) -> HttpResponse;

#[derive(Default)]
pub struct HttpServer {
    // TODO: router struct?
    handlers: HashMap<(String, RequestMethod), Box<Handler>>,
    default_handler: Option<Box<Handler>>,
}

// TODO: some macro(?) like #[get("/")] or #[post("/")]
// fn index() -> HttpResponse {
// will make the function into a struct that impls ToHandler

// TODO: doc comments
macro_rules! method {
    ($method:ident, $request_method:ident) => {
        pub fn $method<T>(self, path: &str, handler: T) -> Self
        where
            T: ToHandler,
        {
            self.route(path, RequestMethod::$request_method, handler)
        }
    };
}

impl HttpServer {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_request_handler(&self, req: &HttpRequest) -> Option<&Handler> {
        let boxed_hander_opt = self.handlers.get(&(req.uri.clone(), req.method));

        // Default handler if no matching handler
        let boxed_hander_opt = boxed_hander_opt.or(self.default_handler.as_ref());

        // Extract the handler from the Box
        let handler_opt = boxed_hander_opt.map(|boxed_handler| {
            let handler: &Handler = boxed_handler.as_ref();
            handler
        });

        handler_opt
    }

    fn handle_connection(&self, mut stream: &mut TcpStream) -> std::io::Result<()> {
        let mut reader = BufReader::new(&mut stream);

        let mut buf = String::new();

        loop {
            let read_bytes = reader.read_line(&mut buf)?;
            if read_bytes <= constants::CRLF.len() {
                break;
            }
        }

        if buf.is_empty() {
            return Ok(());
        }

        debug!(
            "bytes read from connection with {}: {}",
            stream.peer_addr()?,
            buf.len()
        );

        let Some(req) = HttpRequest::parse_request(buf) else {
            return Ok(());
        };
        dbg!(&req);

        let Some(handler) = self.get_request_handler(&req) else {
            debug!("No handler found for URI: {}", req.uri);
            return Ok(());
        };

        let response = handler(req);
        dbg!(&response);

        stream.write_fmt(format_args!("{response}"))?;

        Ok(())
    }

    pub fn listen<A: ToSocketAddrs>(&self, addr: A) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr)?;

        info!("listening on {}", listener.local_addr()?);

        // accept connections and process them serially
        for stream in listener.incoming() {
            let mut stream = stream?;
            self.handle_connection(&mut stream)?;
        }

        Ok(())
    }

    pub fn route<T>(mut self, path: &str, method: RequestMethod, handler: T) -> Self
    where
        T: ToHandler,
    {
        self.handlers
            .insert((path.to_string(), method), handler.to_handler());
        self
    }

    method!(get, GET);
    method!(post, POST);
    method!(put, PUT);
    method!(delete, DELETE);
    method!(head, HEAD);
    method!(options, OPTIONS);
    method!(connect, CONNECT);
    method!(trace, TRACE);
    method!(patch, PATCH);

    pub fn default_handler<T>(mut self, handler: T) -> Self
    where
        T: ToHandler,
    {
        self.default_handler = Some(handler.to_handler());
        self
    }
}

pub trait ToHandler {
    fn to_handler(self) -> Box<Handler>;
}

impl<T, B> ToHandler for T
where
    T: Fn(HttpRequest) -> B + 'static,
    B: Into<HttpResponse>,
{
    fn to_handler(self) -> Box<Handler> {
        Box::new(move |req| self(req).into())
    }
}

// TODO: funcs that return Result<JSON, Error(?)>
// Really we don't have to do that here, we just need to
// impl Into<HttpResponse> for JSON
// But we DO need to handle funcs that return Result
// actually maybe not, kinda cba

// TODO?: serde, maybe make our own ToJson trait so user
// can use any json lib they want - that we support (with feature flags)
