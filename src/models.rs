use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShortenUrl {
    pub original_url: String,
    pub shorten_url: String,
}

impl ShortenUrl {
    pub fn new(original_url: String) -> Self {
        let shorten_url = uuid::Uuid::new_v4().to_string();
        Self {
            original_url,
            shorten_url
        }
    }
}
