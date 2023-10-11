use axum::http;
use axum::routing::{get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 3000;

    let address: String = format!("0.0.0.0:{port}");

    let app = Router::new().route("/", get(heartbeat));

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn heartbeat() -> http::StatusCode {
    http::StatusCode::OK
}
