use crate::responder::ApiResponder;
use rocket::http::Status;
use serde_json::json;

#[get("/hello_world")]
pub fn hello_world() -> ApiResponder {
    ApiResponder::new(
        Status::Ok,
        json!({
            "message": "Hello, World"
        }),
    )
}
