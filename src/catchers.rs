use crate::errors::{DefaultApiError, InternalServerError, NotFoundError};
use crate::responder::ApiResponder;
use rocket::{Catcher, Request};
use colored::Colorize;

#[catch(404)]
fn not_found(req: &Request) -> ApiResponder {
    let uri = req.uri().to_string().green();
    let method = req.method().to_string().yellow();
    NotFoundError::new(
        format!("Client requested unhandled method {method} for {uri}"),
        "Sorry. The server was unable to found what are you looking for."
    )
}

#[catch(500)]
fn internal_error() -> ApiResponder {
    InternalServerError::new(
        "A unknow error occurred during a request.",
        "Sorry. A unknow internal error occurred."
    )
}

pub fn build_catchers() -> Vec<Catcher> {
    catchers![
        not_found,
        internal_error
    ]
}
