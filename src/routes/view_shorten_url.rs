use crate::database::Db;
use crate::models::ShortenUrl;
use crate::responder::{ApiResponder, ApiResponse};
use rocket::futures::TryFutureExt;
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection};
use serde_json::json;

#[get("/view/<shorten>")]
pub async fn view_shorten_url(mut db: Connection<Db>, shorten: String) -> ApiResponse {
    let values = sqlx::query!(
        "SELECT original, shorten FROM urls WHERE shorten = ?",
        shorten
    )
    .fetch_one(&mut **db)
    .map_err(|err| {
        eprintln!("{err}");
        ApiResponder::new(
            Status::InternalServerError,
            json!({
                "err": "Could not get the shorten url due to server malfunction"
            }),
        )
    })
    .await?;
    let shorten_url = ShortenUrl {
        original_url: values.original,
        shorten_url: values.shorten,
    };
    Ok(ApiResponder::new(
        Status::Ok,
        json!({
            "message": "Sucessfully founded the url.",
            "shorten_url": shorten_url
        }),
    ))
}
