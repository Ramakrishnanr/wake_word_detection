// Actix web related
use actix_web::{Responder, web};

pub async fn predict_wake_word(web::Path(word) : web::Path<String>) -> impl Responder {
    // TODO: Add business logic here after successful compilation.
    println!("{:?}", word);
    return web::Json(word)
}
