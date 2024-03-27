use rocket_db_pools::Database;
use rocket::{Rocket, Build};

use crate::{routes, catchers, database};

pub fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .attach(database::Db::init())
        .mount("/", routes::build_routes())
        .register("/", catchers::build_catchers())
}
