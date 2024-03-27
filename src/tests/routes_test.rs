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

#[test]
fn non_existing_route_should_return_404() {
    let rocket = build_rocket(); 
    let client = Client::tracked(rocket).unwrap();
    let res = client.get("/random_route").dispatch();
    assert_eq!(res.status(), Status::NotFound);
}

#[test]
fn create_url_without_parameter_should_return_400() {
    let rocket = build_rocket(); 
    let client = Client::tracked(rocket).unwrap();
    let res = client.post("/new?url=").dispatch();
    assert_eq!(res.status(), Status::BadRequest);
}

#[test]
fn create_url_with_bad_parameter_should_return_400() {
    let rocket = build_rocket(); 
    let client = Client::tracked(rocket).unwrap();
    let res = client.post("/new?url=invalid_url").dispatch();
    assert_eq!(res.status(), Status::BadRequest);
}

#[test]
fn create_url_good_parameter_should_return_201() {
    let rocket = build_rocket(); 
    let client = Client::tracked(rocket).unwrap();
    let url = "https://www.google.com";
    let res = client.post(format!("/new?url={url}")).dispatch();
    assert_eq!(res.status(), Status::Created);
}
