mod create_shorten_url;
mod delete_url;
mod hello_world;
mod view_shorten_url;
mod redirect_to;

pub fn build_routes() -> Vec<rocket::Route> {
    routes![
        hello_world::hello_world,
        create_shorten_url::create_shorten_url,
        view_shorten_url::view_shorten_url,
        delete_url::delete_url,
        redirect_to::redirect_to,
    ]
}
