use rocket::{http::Status, local::blocking::Client};
use serde_json::json;

#[test]
fn hello_route_should_return_200() {
    let rocket = rocket::build();
    let client = Client::tracked(rocket).unwrap();
    let res = client.get("/hello_world").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.into_string().unwrap(), json!({ "message": "Hello, World"}).to_string());
}
