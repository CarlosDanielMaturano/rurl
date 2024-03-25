mod create_shorten_url;
mod hello_world;
mod view_shorten_url;
use rocket::Route;

pub fn build_routes() -> Vec<Route> {
    routes![
        hello_world::hello_world,
        create_shorten_url::create_shorten_url,
        view_shorten_url::view_shorten_url,
    ]
}
