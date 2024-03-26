use crate::database::Db;
use crate::responder::{ApiResponder, ApiResponse};
use crate::models::ShortenUrl;
use rocket::futures::TryFutureExt;
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection};
use serde_json::json;

#[get("/view/<hash>")]  
pub async fn view_shorten_url(mut db: Connection<Db>, hash: String) -> ApiResponse {
    let values = sqlx::query!(
        "SELECT url, hash FROM urls WHERE hash = ?",
        hash
    )
    .fetch_optional(&mut **db)
    .map_err(|err| {
        eprintln!("{err}");
        ApiResponder::new(
            Status::InternalServerError,         
            json!({
                "message": "Could not get the shorten url due to server malfunction",
                "err": "Internal Server Error"
            })
        )
    })
    .await?
    .ok_or_else(||
        ApiResponder::new(
            Status::NotFound,
            json!({
                "message": "Could not find a shorten url with the given hash",
                "err": "Not Found"
            })
        )
    )?;

    let shorten_url = ShortenUrl {
        url: values.url,
        hash: values.hash
    };
    Ok(ApiResponder::new(
        Status::Ok,
        json!({
            "message": "Sucessfully founded the url.",
            "shorten_url": shorten_url
        })
    ))
}
