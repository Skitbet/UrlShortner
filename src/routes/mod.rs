use actix_web::web;
pub mod redirect;
pub mod shorten;
pub fn get_services() -> actix_web::Scope {
    web::scope("")
        .service(shorten::shorten)
        .service(redirect::redirect)
}