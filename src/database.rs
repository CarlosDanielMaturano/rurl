use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("urls")]
pub struct Db(sqlx::SqlitePool);
