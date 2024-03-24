#[macro_use] 
extern crate rocket;

mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes::build_routes())
        .launch()
        .await;
    Ok(()) 
}
