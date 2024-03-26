use crate::responder::ApiResponder;
use core::fmt::Display;
use rocket::http::Status;
use serde_json::json;

pub trait DefaultApiError {
    const STATUS: Status;
    fn respond(message: &str) -> ApiResponder {
        ApiResponder::new(
            Self::STATUS,
            json!({
                "err": Self::STATUS.to_string(),
                "message": message
            }),
        )
    }
    fn log<T: Display>(err: T) {
        eprintln!("Server error: {err}")
    }
    fn new<T: Display>(err: T, message: &str) -> ApiResponder {
        Self::log(err);
        Self::respond(message)
    }
}

pub struct InternalServerError;
impl DefaultApiError for InternalServerError {
    const STATUS: Status = Status::InternalServerError;
}

pub struct NotFoundError;
impl DefaultApiError for NotFoundError {
    const STATUS: Status = Status::NotFound;
}

pub struct BadRequestError;
impl DefaultApiError for BadRequestError {
    const STATUS: Status = Status::BadRequest;
}
