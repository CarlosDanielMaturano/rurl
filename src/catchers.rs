use crate::errors::{DefaultApiError, InternalServerError, NotFoundError};
use crate::responder::ApiResponder;
use rocket::{Catcher, Request};

#[catch(404)]
fn not_found(req: &Request) -> ApiResponder {
    let uri = req.uri();
    let method = req.method();
    NotFoundError::new(
        format!("No matching routes for {method} {uri}"),
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
