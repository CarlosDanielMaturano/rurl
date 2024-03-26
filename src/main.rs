#[macro_use]
extern crate rocket;

mod database;
mod errors;
mod models;
mod responder;
mod routes;
mod catchers;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(crate::database::Db::init())
        .mount("/", routes::build_routes())
        .register("/", catchers::build_catchers())
        .launch()
        .await;
    Ok(())
}
