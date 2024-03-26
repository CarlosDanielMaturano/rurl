use crate::database::Db;
use crate::errors::{BadRequestError, DefaultApiError, InternalServerError};
use crate::models::ShortenUrl;
use crate::responder::{ApiResponder, ApiResponse};
use core::fmt;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket_db_pools::{sqlx, Connection};
use serde_json::json;

#[derive(Deserialize)]
pub struct Url {
    pub value: String,
}

#[post("/new?<url>")]
pub async fn create_shorten_url(mut db: Connection<Db>, url: Option<String>) -> ApiResponse {
    let url = url.ok_or(BadRequestError::new(
            "Missing url parameter",
            "Provide a url parameter. E.g ?url=https//www.google.com"
    ))?;

    let shorten_url = ShortenUrl::new(url);

    sqlx::query!(
        "INSERT INTO urls (url, hash) VALUES (?, ?)",
        shorten_url.url,
        shorten_url.hash
    )
    .execute(&mut **db)
    .await
    .map_err(|err| {
        InternalServerError::new(
            err,
            "Could not create a new shorten url due to server malfunction",
        )
    })?;

    Ok(ApiResponder::new(
        Status::Ok,
        json!({
            "message": "Sucessfully created a new shorten url",
            "shorten_url": shorten_url
        }),
    ))
}
