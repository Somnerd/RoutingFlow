use dotenvy::dotenv;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).ok();

    dotenv().ok();
    let host = std::env::var("WORKER_DB_HOST").unwrap_or_else(|_| "db".into());
    let port = std::env::var("WORKER_DB_PORT").unwrap_or_else(|_| "5432".into());
    let name = std::env::var("WORKER_DB_NAME").unwrap_or_else(|_| "deluxe".into());
    let user = std::env::var("WORKER_DB_USER").unwrap_or_else(|_| "deluxe".into());
    let pass = std::env::var("WORKER_DB_PASSWORD").unwrap_or_else(|_| "deluxe".into());

    let conn_str = format!("host={} port={} dbname={} user={} password={}", host, port, name, user, pass);
    info!("Worker starting. Connecting to Postgres at {}:{} ...", host, port);

    let (client, connection) = match tokio_postgres::connect(&conn_str, tokio_postgres::NoTls).await {
        Ok((c, conn)) => (c, conn),
        Err(e) => {
            error!("Failed to connect to Postgres: {e}");
            loop {
                sleep(Duration::from_secs(3)).await;
                info!("Retrying Postgres connection...");
                if let Ok((c, conn)) = tokio_postgres::connect(&conn_str, tokio_postgres::NoTls).await {
                    info!("Connected to Postgres.");
                    let _ = tokio::spawn(conn);
                    run_loop(&c).await;
                    return;
                }
            }
        }
    };
    tokio::spawn(connection);
    run_loop(&client).await;
}

async fn run_loop(_client: &tokio_postgres::Client) {
    loop {
        info!("worker: heartbeat");
        sleep(Duration::from_secs(5)).await;
    }
}
