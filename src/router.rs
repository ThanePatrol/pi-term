use std::io;
use axum::{Form, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, get_service, post};
use tower_http::services::ServeDir;
use crate::{DeviceData, shell_cmds};

pub fn build_routes() -> Router {
    let serve_dir = get_service(ServeDir::new("assets")).handle_error(handle_error);

    Router::new()
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
        .route("/choose_terminal", post(update_terminal))
        .route("/add_terminal", post())
}

pub async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub async fn update_terminal(Form(form_data): Form<DeviceData>) -> impl IntoResponse {
    println!("form data: {:?}", form_data);
    shell_cmds::init_new_web_terminal(&form_data.node_ip, form_data.port)
        .expect("Error creating terminal - todo - error handling");

    shell_cmds::start_ssh_session_in_ttyd(&form_data.node_ip, form_data.port, &form_data.user)
        .unwrap();
    StatusCode::OK
}

