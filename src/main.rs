use std::error::Error;
use std::net::SocketAddr;
use axum::http::{StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::{body, Form, Json, Router};
use axum::body::Full;
use axum::routing::{get, post};
pub use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    //init environment variables
    dotenvy::from_path(".env").unwrap();

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", address);

    axum::Server::bind(&address)
        .serve(build_routes().into_make_service())
        .await?;
    Ok(())
}

fn build_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/terminal", get(serve_terminal))
        .route("/choose_terminal",post(update_terminal))
}

async fn serve_terminal() -> impl IntoResponse {
    let path = dotenvy::var("TERMINAL_FILE").unwrap();

    match std::fs::read_to_string(&path) {
        Ok(file) => Html::from(file).into_response(),
        Err(e) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Full::from(e.to_string())))
            .unwrap(),
    }
}

#[derive(Deserialize, Debug)]
struct FormUpdateData {
    node_id: i32,
    port: i32,
    baud_rate: i32,
}

async fn update_terminal(Form(form_data): Form<FormUpdateData>) -> impl IntoResponse {
    println!("form data: {:?}", form_data);

    StatusCode::OK
}