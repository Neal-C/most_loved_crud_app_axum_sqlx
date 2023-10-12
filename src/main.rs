mod handlers;

use axum::routing::{get, Router};
use sqlx::postgres::PgPoolOptions;

const MAX_PG_CONNECTIONS: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: String = std::env::var("PORT").unwrap_or_else(|_| String::from("3000"));

    let address: String = format!("0.0.0.0:{port}");

    let database_url: String =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");

    let pool = PgPoolOptions::new()
        .max_connections(MAX_PG_CONNECTIONS)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(handlers::heartbeat))
        .with_state(pool);

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
