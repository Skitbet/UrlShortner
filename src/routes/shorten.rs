use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::{req_params, state, Url};
use crate::utils;
use crate::utils::errors::HttpError;

#[post("/shorten")]
async fn shorten(app_data: web::Data<state::AppState>, req: web::Json<req_params::ShortenReq>) -> Result<impl Responder, HttpError> {
    let mut db = &app_data.database;

    if (req.url.is_empty()) {
        return Err(HttpError::InvalidParams);
    }

    let short_code = utils::generate_short_code();
    let url = Url {
        short_code,
        original_url: req.url.clone(),
    };

    let collection = db.collection::<Url>("urls");
    if let Err(_) = collection.insert_one(&url).await {
        return Err(HttpError::InternalError);
    }
    Ok(web::Json(url))
}

// pub async fn shorten_url(
//     Json(payload): Json<Url>,            // Deserialize the payload into a Url
//     State(db): State<Arc<Database>>,     // Extract the database state
// ) -> AxumJson<Url> {                    // Return a Json response with Url
//     if payload.original_url.is_empty() {
//         // If the URL is empty, return a simple error message as a JSON response
//         return AxumJson(Url {
//             short_code: "".to_string(),
//             original_url: "URL cannot be empty".to_string(),
//         });
//     }
//
//     let short_code = utils::generate_short_code();  // Generate a short code
//     let url = Url {
//         short_code,
//         original_url: payload.original_url,
//     };
//
//     let collection = db.collection::<Url>("urls");
//     // Attempt to insert the URL into the database, but handle errors silently
//     if let Err(_) = collection.insert_one(&url).await {
//         // If there's a database error, return a generic error message
//         return AxumJson(Url {
//             short_code: "".to_string(),
//             original_url: "Failed to save URL".to_string(),
//         });
//     }
//
//     // Return the shortened URL
//     AxumJson(url)
// }
