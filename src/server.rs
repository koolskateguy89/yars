#![allow(dead_code)]

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::constants;

type Handler = dyn Fn();

#[derive(Default)]
pub struct HttpServer {
    handlers: HashMap<String, Box<Handler>>,
}

impl HttpServer {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: handle HTTP header - header is wrong word
    // more like overhead, idrk
    // i think metadata is best word

    // TODO (next!!!): be able to respond

    fn handle_client(&self, stream: TcpStream) -> std::io::Result<()> {
        let mut reader = BufReader::new(stream);

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

    pub fn get(&self, path: &str, _handler: impl Fn() -> ()) -> std::io::Result<()> {
        todo!()
    }

    pub fn post(&self, path: &str, _handler: impl Fn() -> ()) -> std::io::Result<()> {
        todo!()
    }
}
