use crate::build_rocket::build_rocket;
use rocket::{http::Status, local::blocking::Client};
use serde_json::json;

#[test]
fn hello_route_should_return_200() {
    let rocket = build_rocket(); 
    let client = Client::tracked(rocket).unwrap();
    let res = client.get("/hello_world").dispatch();
    assert_eq!(res.status(), Status::Ok);
    let expected_json = json!({ 
        "message": "Hello, World",
        "status": 200
    });
    assert_eq!(res.into_string().unwrap(), expected_json.to_string());
}
