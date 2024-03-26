use crate::database::Db;
use crate::responder::ApiResponder;
use crate::errors::{DefaultApiError, InternalServerError, NotFoundError};
use rocket::response::Redirect;
use rocket::futures::TryFutureExt;
use rocket_db_pools::{sqlx, Connection};


#[get("/redirect/<hash>")]
pub async fn redirect_to(mut db: Connection<Db>, hash: String) -> Result<Redirect, ApiResponder> {
    let url = sqlx::query!("SELECT url  FROM urls WHERE hash = ?", hash)
        .fetch_optional(&mut **db)
        .map_err(|err| {
            InternalServerError::new(err, "Could not get the url due to server malfunction.")
        })
        .await?
        .ok_or_else(|| {
            NotFoundError::new(
                "Query returned none",
                "Could not find a shorten url with the given hash",
            )
        })?;
    Ok(Redirect::to(url.url))
}

