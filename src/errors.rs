use crate::responder::ApiResponder;
use core::fmt::Display;
use rocket::http::Status;
use serde_json::json;
use log::{warn, Level};
use colored::Colorize;

pub trait DefaultApiError {
    const STATUS: Status;
    const LOG_LEVEL: Level;
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
        match Self::LOG_LEVEL {
            Level::Warn => warn!("{err}"),
            Level::Error => error!("{err}"),
            _ => eprintln!("{}: {err}", "Err".red())
        }
    }
    fn new<T: Display>(err: T, message: &str) -> ApiResponder {
        Self::log(err);
        Self::respond(message)
    }
}

pub struct InternalServerError;
impl DefaultApiError for InternalServerError {
    const STATUS: Status = Status::InternalServerError;
    const LOG_LEVEL: Level = Level::Error;
}

pub struct NotFoundError;
impl DefaultApiError for NotFoundError {
    const STATUS: Status = Status::NotFound;
    const LOG_LEVEL: Level = Level::Warn;
}

pub struct BadRequestError;
impl DefaultApiError for BadRequestError {
    const LOG_LEVEL: Level = Level::Warn;
    const STATUS: Status = Status::BadRequest;
}
