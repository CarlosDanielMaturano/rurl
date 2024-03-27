#[macro_use]
extern crate rocket;

mod database;
mod errors;
mod models;
mod responder;
mod routes;
mod catchers;
mod logger;
mod build_rocket;

#[cfg(test)]
mod tests;

use build_rocket::build_rocket;
use colored::Colorize;
use log::warn;

#[rocket::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    logger::setup_logger()
        .map_err(|err| format!("Could not initialize the logger. Error: {err}"))?;

    let rocket = build_rocket()
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
        .map_err(|err| format!("Could not launch a rocket instance. Error: {err}"))?;
    Ok(())
}
