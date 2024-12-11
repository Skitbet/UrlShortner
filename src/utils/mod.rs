pub mod errors;

use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}
