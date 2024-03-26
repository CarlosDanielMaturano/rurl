use rocket::serde::{Deserialize, Serialize};
use std::hash::{BuildHasher, RandomState};

#[derive(Serialize, Deserialize)]
pub struct ShortenUrl {
    pub url: String,
    pub hash: String,
}

impl ShortenUrl {
    pub fn new(url: String) -> Self {
        let hash = RandomState::new().hash_one(&url).to_string();
        Self {
            url,
            hash,
        }
    }
}
