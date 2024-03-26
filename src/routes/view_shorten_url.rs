use crate::database::Db;
use crate::errors::{DefaultApiError, InternalServerError, NotFoundError};
use crate::models::ShortenUrl;
use crate::responder::{ApiResponder, ApiResponse};
use rocket::futures::TryFutureExt;
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection};
use serde_json::json;

#[get("/view/<hash>")]
pub async fn view_shorten_url(mut db: Connection<Db>, hash: String) -> ApiResponse {
    let values = sqlx::query!("SELECT url, hash FROM urls WHERE hash = ?", hash)
        .fetch_optional(&mut **db)
        .map_err(|err| {
            InternalServerError::new(err, "Could not get the url deu to server malfunction")
        })
        .await?
        .ok_or_else(|| {
            NotFoundError::new(
                "Query returned none",
                "Could not find a shorten url with the given hash",
            )
        })?;

    let shorten_url = ShortenUrl {
        url: values.url,
        hash: values.hash,
    };
    Ok(ApiResponder::new(
        Status::Found,
        json!({
            "message": "Sucessfully founded the url.",
            "shorten_url": shorten_url
        }),
    ))
}
