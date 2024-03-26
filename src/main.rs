#[macro_use]
extern crate rocket;
mod database;
mod errors;
mod models;
mod responder;
mod routes;
mod catchers;
mod logger;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() -> Result<(), String> {
    logger::setup_logger()
        .map_err(|err| format!("Could not initialize the logger. Error: {err}"))?;
    let _rocket = rocket::build()
        .attach(crate::database::Db::init())
        .mount("/", routes::build_routes())
        .register("/", catchers::build_catchers())
        .launch()
        .await
        .map_err(|err| format!("Could not initalize a rocket instance. Error: {err}"))?;
    Ok(())
}
