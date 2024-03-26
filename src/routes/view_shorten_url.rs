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
    .fetch_one(&mut **db)
    .map_err(|err| {
        eprintln!("{err}");
        ApiResponder::new(
            Status::InternalServerError,         
            json!({
                "err": "Could not get the shorten url due to server malfunction"
            })
        )
    })
    .await?;
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
