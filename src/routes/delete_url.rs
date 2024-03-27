use crate::database::Db;
use crate::errors::{DefaultApiError, InternalServerError, NotFoundError};
use crate::responder::{ApiResponder, ApiResponse};
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection};
use serde_json::json;

#[delete("/delete/<hash>")]
pub async fn delete_url(mut db: Connection<Db>, hash: String) -> ApiResponse {
    sqlx::query!("DELETE FROM urls where hash = ? RETURNING id", hash)
        .fetch_optional(&mut **db)
        .await
        .map_err(|err| {
            InternalServerError::new(err, "Could not delete the url due to server malfuction.")
        })?
        .ok_or_else(||
            NotFoundError::new(
                "Query returned none",
                "Could not find a shorten url with the given hash",
            )
        )?;

    Ok(ApiResponder::new(
        Status::Ok,
        json!({
            "message": "Sucessfully deleted the url."
        }),
    ))
}
