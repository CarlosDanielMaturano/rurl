#[macro_use] 
extern crate rocket;

mod routes;
mod database;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(crate::database::Db::init())
        .mount("/", routes::build_routes())
        .launch()
        .await;
    Ok(()) 
}
