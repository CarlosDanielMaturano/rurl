use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ShortenUrl { 
    original_url: String,
    shorten_url: String
}


