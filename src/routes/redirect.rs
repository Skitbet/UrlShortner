use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use crate::models::state::AppState;
use crate::models::Url;
use crate::utils::errors::HttpError;

#[get("/{short_code}")]
async fn redirect(
    app_data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, HttpError> {
    let short_code = path.into_inner();
    let db = app_data.database.get_database();

    let collection = db.collection::<Url>("urls");

    match collection.find_one(doc! { "short_code": &short_code }).await {
        Ok(Some(url)) => Ok(HttpResponse::Found()
            .insert_header(("Location", url.original_url))
            .finish()),
        Ok(None) => Err(HttpError::NotFound),
        Err(_) => Err(HttpError::InternalError),
    }
}
