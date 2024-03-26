use crate::database::Db;
use crate::errors::{DefaultApiError, InternalServerError};
use crate::models::ShortenUrl;
use crate::responder::{ApiResponder, ApiResponse};
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::{sqlx, Connection};
use serde_json::json;

#[derive(Deserialize)]
pub struct Url {
    pub value: String,
}

#[post("/new", format = "json", data = "<url>")]
pub async fn create_shorten_url(mut db: Connection<Db>, url: Option<Json<Url>>) -> ApiResponse {
    let url = url
        .ok_or(ApiResponder::new(
            Status::BadRequest,
            json!({
                "message": "Could not extract a valid url due to malformed body",
                "err": "Bad Request"
            }),
        ))?
        .into_inner()
        .value;

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
