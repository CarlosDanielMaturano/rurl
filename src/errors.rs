use crate::responder::ApiResponder;
use core::fmt::Display;
use rocket::http::Status;
use serde_json::json;

pub trait DefaultApiError {
    const STATUS: Status;
    const ERR_MESSAGE: &'static str;
    fn respond(message: &str) -> ApiResponder {
        ApiResponder::new(
            Self::STATUS,
            json!({
                "err": Self::ERR_MESSAGE,
                "message": message
            })
        )
    }
    fn log<T: Display>(err: T) {
        eprintln!("Server error: {err}")
    }
    fn new<T: Display>(err: T, message: &str)  -> ApiResponder {
        Self::log(err);
        Self::respond(message)
    }
}

pub struct InternalServerError {}

impl DefaultApiError for InternalServerError {
    const STATUS: Status = Status::InternalServerError;
    const ERR_MESSAGE: &'static str = "Internal server error";
}

