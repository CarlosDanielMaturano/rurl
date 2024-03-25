use rocket_db_pools::{Database, sqlx};

#[derive(Database)]
#[database("urls")]
pub struct Db(sqlx::SqlitePool);

