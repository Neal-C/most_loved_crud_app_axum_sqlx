#![allow(clippy::needless_return)]

mod handlers;

use axum::routing::{delete, get, patch, post, Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

const MAX_PG_CONNECTIONS: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("ENV").is_err() {
        // No file on prod
        dotenv().ok();
    }
    let port: String = std::env::var("PORT").unwrap_or_else(|_| String::from("3000"));

    let address: String = format!("0.0.0.0:{port}");

    // docker run --name db-axum-sqlx -p 5555:5432 -e POSTGRES_PASSWORD=<yourPW> --detach postgres
    let database_url: String =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");

    let pool = PgPoolOptions::new()
        .max_connections(MAX_PG_CONNECTIONS)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(handlers::heartbeat))
        .route("/quote", post(handlers::create_quote))
        .route("/quote", get(handlers::read_quotes))
        .route("/quote", patch(handlers::update_quote))
        .route("/quote", delete(handlers::delete_quote))
        .with_state(pool);

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
