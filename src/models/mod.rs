pub mod state;
pub mod req_params;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub short_code: String,
    pub original_url: String,
}
