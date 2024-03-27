#[macro_use]
extern crate rocket;

mod database;
mod errors;
mod models;
mod responder;
mod routes;
mod catchers;
mod logger;

#[cfg(test)]
mod tests;

use log::warn;
use rocket_db_pools::Database;
use colored::Colorize;

#[rocket::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    logger::setup_logger()
        .map_err(|err| format!("Could not initialize the logger. Error: {err}"))?;
    let rocket = rocket::build()
        .attach(crate::database::Db::init())
        .mount("/", routes::build_routes())
        .register("/", catchers::build_catchers())
        .ignite()
        .await
        .map_err(|err| format!("Could not ignite a rocket instance Error: {err}"))?;

    let port = rocket.config().port;
    let addr = rocket.config().address;
    let host = format!("http://{addr}:{port}").bold().underline().green();
    warn!("Application running on {}", host);

    rocket
        .launch()
        .await
        .map_err(|err| format!("Could not initalize a rocket instance. Error: {err}"))?;
    Ok(())
}
