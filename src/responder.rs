use rocket::response::{self, Responder, Response};
use rocket::{http::Status, Request};
use serde_json::Value;
use std::io::Cursor;

pub struct ApiResponder {
    status: Status,
    body: Value,
}

pub type ApiResponse = Result<ApiResponder, ApiResponder>;

impl ApiResponder {
    pub fn new(status: Status, body: Value) -> Self {
        Self { status, body }
    }
}

impl<'r> Responder<'r, 'static> for ApiResponder {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = self.body.to_string();
        Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(self.status)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}
