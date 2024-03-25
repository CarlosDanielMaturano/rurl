use crate::database::Db;
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
pub async fn create_shorten_url(mut db: Connection<Db>,url: Option<Json<Url>>) -> ApiResponse {
    let url = url.ok_or(ApiResponder::new(
        Status::BadRequest,
        json!({ "err": "Could not extract a valid url due to malformed body"}),
    ))?.into_inner().value;

    let shorten_url = ShortenUrl::new(url);

    sqlx::query!(
        "INSERT INTO urls (original, shorten) VALUES (?, ?)",
         shorten_url.original_url,shorten_url.shorten_url
    )
    .execute(&mut **db)
    .await
    .map_err(|err| {
        eprintln!("{err}");
        ApiResponder::new(
            Status::InternalServerError, 
            json!({ "err": "Could not create a new shorte url due to server malfunction"}
        ))
    })?;

    Ok(ApiResponder::new(
        Status::Ok, 
        json!({
            "message": "Sucessfully created a new shorten url",
            "url": shorten_url
        })
    ))
}
