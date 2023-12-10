#![allow(dead_code)]

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::constants;
use crate::request::{HTTPRequest, RequestMethod};
use crate::response::HTTPResponse;

type Handler = dyn Fn(HTTPRequest) -> HTTPResponse;

#[derive(Default)]
pub struct HttpServer {
    // TODO: router struct?
    // TODO?: change String to Box<str>?
    handlers: HashMap<(String, RequestMethod), Box<Handler>>,
    default_handler: Option<Box<Handler>>,
}

// TODO: some macro(?) like #[get("/")] or #[post("/")]
// fn index() -> HTTPResponse {
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

    // TODO: handle HTTP header - header is wrong word
    // more like overhead, idrk
    // i think metadata is best word

    // TODO (ACTUALLY next): handle & parse HTTP request
    // TODO: use nom
    // TODO (next!!!): be able to respond

    fn handle_client(&self, stream: TcpStream) -> std::io::Result<()> {
        let mut reader = BufReader::new(stream);

        // TODO: check handlers

        loop {
            let mut buf = String::new();
            let read_bytes = reader.read_line(&mut buf)?;
            println!(
                "Read {read_bytes} bytes from stream = {}",
                buf.escape_default(),
            );

            if read_bytes <= constants::CRLF.len() {
                break;
            }
        }

        Ok(())
    }

    // TODO: make addr type the same as TcpListener::bind
    pub fn listen(&self, addr: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr)?;

        println!("Listening on {}", listener.local_addr()?);

        // accept connections and process them serially
        for stream in listener.incoming() {
            self.handle_client(stream?)?;
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
    // method!(connect, CONNECT);
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
    T: Fn(HTTPRequest) -> B + 'static,
    B: Into<HTTPResponse>,
{
    fn to_handler(self) -> Box<Handler> {
        Box::new(move |req| self(req).into())
    }
}

// TODO: funcs that return Result<JSON, Error(?)>

// TODO?: serde, maybe make our own ToJson trait so user
// can use any json lib they want - that we support (with feature flags)
