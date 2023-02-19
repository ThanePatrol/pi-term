use std::error::Error;
use std::net::SocketAddr;
use std::process::{Command, Output};
use axum::http::{StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::{body, Form, Router};
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
struct FormData {
    node_ip: String,
    port: i32,
    baud_rate: i32,
}

async fn update_terminal(Form(form_data): Form<FormData>) -> impl IntoResponse {
    println!("form data: {:?}", form_data);
    //init the terminal
    init_web_terminal(&form_data)
        .expect("Error creating terminal - todo - error handling");

    StatusCode::OK
}

///Flow is:
///     1. Kill old tmux instances
///     2. Create Tmux container
///     3. Attach Tmux container
///     4. Start ttyd on specified port
/// Tmux containers names are the ip addresses with dots removed
fn init_web_terminal(form: &FormData) -> Result<(), Box<dyn Error>> {
    fn start_minicom(baud_rate: i32) -> std::io::Result<Output> {
        Command::new("minicom")
            .arg("-b")
            .arg(baud_rate.to_string())
            .args(["-D", "/dev/ttyUSB0"])
            .output()
    }

    fn start_tmux(node_id: &String, port: i32, baud_rate: i32) -> std::io::Result<Output> {
        Command::new("python3")
            .arg("./backend/python_scripts/tmux_init.py")
            .arg(node_id)
            .arg(port.to_string())
            .arg(baud_rate.to_string())
            .output()
    }


    let ip_without_dots = &form.node_ip.clone().replace(".", "");

    let _ = start_tmux(ip_without_dots, form.port, form.baud_rate)
        .expect("Error running python script for init of tmux and ttyd");


    Ok(())
}