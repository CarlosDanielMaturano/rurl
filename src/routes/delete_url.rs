use crate::database::Db;
use crate::responder::{ApiResponder, ApiResponse};
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection};
use serde_json::json;

#[delete("/delete/<hash>")]
pub async fn delete_url(mut db: Connection<Db>, hash: String) -> ApiResponse {
    sqlx::query!("DELETE FROM urls where hash = ?", hash)
        .execute(&mut **db)
        .await
        .map_err(|err| {
            eprintln!("{err}");
            ApiResponder::new(
                Status::InternalServerError,
                json!({
                    "err": "Could not delete the url due to server malfuction."
                }),
            )
        })?;
    Ok(ApiResponder::new(
        Status::Ok,
        json!({
            "message": "Sucessfully deleted the url."
        })
    ))
}
