use crate::responder::ApiResponder;
use core::fmt::Display;
use rocket::http::Status;
use serde_json::json;


pub trait DefaultApiError {
    fn respond(message: &str) -> ApiResponder;
    fn log<T: Display>(err: T);
}

pub struct InternalServerError {}

impl DefaultApiError for InternalServerError {
    fn respond(message: &str) -> ApiResponder {
        ApiResponder::new(
            Status::InternalServerError,
            json!({
                "err": "Internal Server Error",
                "message": message
            })
        )
    }
    fn log<T: Display>(err: T) {
        eprintln!("Server error: {err}")
    }
}


impl InternalServerError {
    pub fn log_and_respond<T: Display>(err: T, message: &str)  -> ApiResponder {
        Self::log(err);
        Self::respond(message)
    }
}
