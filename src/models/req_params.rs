use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShortenReq {
    pub url: String,
}