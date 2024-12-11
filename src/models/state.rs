use mongodb::Database;
use crate::db::MongoDB;

pub struct AppState {
    pub database: MongoDB,
}