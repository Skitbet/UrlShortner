use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::{req_params, state, Url};
use crate::utils;
use crate::utils::errors::HttpError;

#[post("/shorten")]
async fn shorten(
    app_data: web::Data<state::AppState>,
    req: web::Json<req_params::ShortenReq>
) -> Result<impl Responder, HttpError> {
    let db = app_data.database.get_database();

    if req.url.is_empty() {
        return Err(HttpError::InvalidParams);
    }

    let short_code = utils::generate_short_code();
    let url = Url {
        short_code: short_code.clone(),
        original_url: req.url.clone(),
    };

    let collection = db.collection::<Url>("urls");
    match collection.insert_one(&url, None).await {
        Ok(_) => Ok(web::Json(url)),
        Err(_) => Err(HttpError::InternalError),
    }
}
