use crate::{build_rocket::build_rocket, models::ShortenUrl};
use rocket::{http::Status, local::blocking::Client};
use serde_json::json;
use serde::{Deserialize, Serialize};

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
fn create_url_with_good_parameter_should_return_201_and_delete_it_should_return_200() {
    #[derive(Serialize, Deserialize)]
    struct ExpectedResponseBody {
        message: String,
        status: i32,
        shorten_url: ShortenUrl
    }

    let rocket = build_rocket(); 
    let client = Client::tracked(rocket).unwrap();
    let url = "https://www.google.com";
    let res = client.post(format!("/new?url={url}")).dispatch();
    let status = res.status();
    let res_body = res.into_json::<ExpectedResponseBody>().unwrap();
    assert_eq!(status, Status::Created);

    // Delete the created url
    let hash = res_body.shorten_url.hash;
    let res = client.delete(format!("/delete/{hash}")).dispatch();
    assert_eq!(res.status(), Status::Ok);
}
