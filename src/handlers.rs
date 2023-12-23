use std::{collections::HashMap, str::FromStr};

use axum::{extract, http};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, FromRow)]
pub struct Quote {
    id: uuid::Uuid,
    book: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
        let now_timestamptz = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            inserted_at: now_timestamptz,
            updated_at: now_timestamptz,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

#[derive(Debug, Deserialize)]
pub struct OptionalQuote {
    book: Option<String>,
    quote: Option<String>,
}

pub async fn heartbeat() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_quote(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateQuote>,
) -> Result<(http::StatusCode, axum::Json<Quote>), http::StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);

    let postgres_query_result: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
        r#"INSERT INTO quote (id, book, quote, inserted_at, updated_at) VALUES ($1,$2,$3,$4,$5)"#,
    )
    .bind(quote.id) // Seemingly not necessary to borrow for Copy & Owned types
    .bind(&quote.book)
    .bind(&quote.quote)
    .bind(quote.inserted_at) // clippy::needless_borrow // Seemingly not necessary to borrow for Copy & Owned types
    .bind(quote.updated_at) // clippy::needless_borrow // Seemingly not necessary to borrow for Copy & Owned types
    .execute(&pool)
    .await;

    match postgres_query_result {
        Ok(_) => return Ok((http::StatusCode::CREATED, axum::Json(quote))),
        Err(_) => return Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn read_quotes(
    extract::State(pool): extract::State<PgPool>,
) -> Result<(http::StatusCode, axum::Json<Vec<Quote>>), http::StatusCode> {
    let postgres_query_result: Result<Vec<Quote>, sqlx::Error> =
        sqlx::query_as::<_, Quote>(r#"SELECT id, book, quote, inserted_at, updated_at FROM quote"#)
            .fetch_all(&pool)
            .await;

    match postgres_query_result {
        Ok(quotes) => return Ok((http::StatusCode::OK, axum::Json(quotes))),
        Err(_) => return Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_quote(
    extract::State(pool): extract::State<PgPool>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
    axum::Json(payload): extract::Json<OptionalQuote>,
) -> Result<(http::StatusCode, axum::Json<Quote>), http::StatusCode> {
    let now_timestamptz = chrono::Utc::now();

    let Some(id) = params.get("id") else {
        return Err(http::StatusCode::BAD_REQUEST);
    };

    let Ok(id) = uuid::Uuid::from_str(id) else {
        return Err(http::StatusCode::BAD_REQUEST);
    };

    let postgres_query_result: Result<Quote, sqlx::Error> = sqlx::query_as::<_, Quote>(
        "
        UPDATE quote SET (quote, updated_at) = ($2,$3) WHERE quote.id = $1 RETURNING *;
        ",
    )
    .bind(id)
    .bind(&payload.quote)
    .bind(now_timestamptz)
    .fetch_one(&pool)
    .await;

    match postgres_query_result {
        Ok(updated_quote) => return Ok((http::StatusCode::OK, axum::Json(updated_quote))),
        Err(sqlx::Error::RowNotFound) => return Err(http::StatusCode::NOT_FOUND),
        Err(sqlx::Error::Database(database_error)) => {
            println!("DATABASE ERROR ----- LOGGED");
            println!("{database_error}");
            return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
        }
        Err(mysterious_error) => {
            println!("MYSTERIOUS ERROR ----- LOGGED");
            println!("{mysterious_error}");
            return Err(http::StatusCode::IM_A_TEAPOT);
        }
    }
}

pub async fn delete_quote(
    extract::State(pool): extract::State<PgPool>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> Result<(http::StatusCode, axum::Json<Quote>), http::StatusCode> {
    let Some(id) = params.get("id") else {
        return Err(http::StatusCode::BAD_REQUEST);
    };

    let Ok(id) = uuid::Uuid::from_str(id) else {
        return Err(http::StatusCode::BAD_REQUEST);
    };

    let postgres_query_result = sqlx::query_as::<_, Quote>(
        "
        DELETE FROM quote WHERE quote.id = $1 RETURNING *
        ",
    )
    .bind(id)
    .fetch_one(&pool)
    .await;

    match postgres_query_result {
        Ok(deleted_quote) => return Ok((http::StatusCode::OK, axum::Json(deleted_quote))),
        Err(sqlx::Error::RowNotFound) => return Err(http::StatusCode::NOT_FOUND),
        Err(_) => return Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
