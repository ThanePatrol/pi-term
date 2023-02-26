mod db;
mod router;
mod shell_cmds;

use crate::router::{build_routes, handle_error, update_terminal};
pub use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //init environment variables
    dotenvy::from_path(".env").unwrap();

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", address);

    axum::Server::bind(&address)
        .serve(build_routes().into_make_service())
        .await?;
    Ok(())
}



#[derive(Deserialize, Debug)]
pub struct DeviceData {
    node_ip: Ipv4Addr,
    port: i32,
    baud_rate: i32,
    user: String,
    tty_path: String,
}
