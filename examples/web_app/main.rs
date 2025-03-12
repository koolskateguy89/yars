//! A simple web app that lets you guess a number. Exposes just one page.
//!
//! The number is randomly generated on startup.
//!
//! The user can keep guessing until they get it right.
//! The server will respond with a message saying if the guess was too high or too low.

use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use yars::{
    http::{HttpRequest, HttpResponse},
    YarsServer,
};

// TODO: read file instead of include so it gets reloaded

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().html(include_str!("res/index.html")))
}

async fn favicon(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", "image/x-icon")
        .body(include_bytes!("res/favicon.ico")))
}

async fn script(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().js(include_str!("res/script.js")))
}

#[derive(Debug, Serialize, Deserialize)]
struct GuessBody {
    guess: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct GuessResponse {
    message: String,
}

fn guess(target: u8) -> impl Fn(HttpRequest) -> Result<HttpResponse> {
    move |req: HttpRequest| -> Result<HttpResponse> {
        use std::cmp::Ordering;

        let body = req.body.unwrap();
        let GuessBody { guess } = serde_json::from_slice(&body)?;
        tracing::debug!("User guessed: {}", guess);

        let message = match guess.cmp(&target) {
            Ordering::Less => "Too low!",
            Ordering::Greater => "Too high!",
            Ordering::Equal => "You guessed it!",
        }
        .to_owned();

        let response = GuessResponse { message };
        let response_bytes = serde_json::to_vec(&response)?;

        Ok(HttpResponse::Ok().json(response_bytes))
    }
}

#[tokio::main]
async fn main() -> yars::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut rng = rand::rng();
    let secret_number: u8 = rng.random();
    tracing::debug!("Secret number: {}", secret_number);

    // TODO?: make a helper func to wrap sync handlers
    let actual_guess_handler = guess(secret_number);
    // having to wrap because of async :(
    let wrapped_guess_handler = move |req| {
        let res = actual_guess_handler(req);
        Box::pin(async move { res })
    };

    YarsServer::default_server()
        .get("/", index)
        .get("/favicon.ico", favicon)
        .get("/script.js", script)
        .post("/guess", wrapped_guess_handler)
        .listen("127.0.0.1:8000")
        .await
}
