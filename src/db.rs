use std::error::Error;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use crate::DeviceData;

pub async fn init_db(db_url: String) -> Result<Pool<Sqlite>, Box<dyn Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;
    Ok(pool)
}

pub async fn insert_into_table(pool: &Pool<Sqlite>, form: &DeviceData) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO devices (node_ip, baud_rate, user, port, tty_path) \
    VALUES (?, ?, ?, ?, ?)")
        .bind(form.node_ip.to_string())
        .bind(form.baud_rate)
        .bind(&form.user)
        .bind(form.port)
        .bind(&form.tty_path)
        .execute(pool)
        .await?;
    Ok(())
}

