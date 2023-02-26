use crate::{shell_cmds, DeviceData, db};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get_service, post};
use axum::{Form, Router};
use sqlx::{Pool, Sqlite};
use std::io;
use axum::extract::State;
use tower_http::services::ServeDir;

pub fn build_routes(pool: Pool<Sqlite>) -> Router {
    let serve_dir = get_service(ServeDir::new("assets")).handle_error(handle_error);

    Router::new()
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
        .route("/choose_terminal", post(update_terminal))
        .route("/add_terminal", post(save_node_details))
        .with_state(pool)
}

pub async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub async fn update_terminal(Form(form_data): Form<DeviceData>) -> impl IntoResponse {
    println!("form data: {:?}", form_data);
    let new_resp = shell_cmds::init_new_web_terminal(&form_data.node_ip, form_data.port);

    let new_ssh =
        shell_cmds::start_ssh_session_in_ttyd(&form_data.node_ip, form_data.port, &form_data.user);

    match new_resp.is_ok() && new_ssh.is_ok() {
        true => StatusCode::OK,
        false => {
            if new_resp.is_ok() {
                println!("Error with ssh init: {:?}", new_ssh);
            } else {
                println!("Error with new terminal init: {:?}", new_resp);
            }
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn save_node_details(
    State(pool): State<Pool<Sqlite>>,
    Form(form_data): Form<DeviceData>,
) -> impl IntoResponse {
    println!("requested to insert {:?}", form_data);
    let res = db::insert_into_table(&pool, &form_data).await;
    match res.is_ok() {
        true => StatusCode::OK,
        false => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
