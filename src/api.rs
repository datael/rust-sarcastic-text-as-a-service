use actix_web::{web, Error, Responder, HttpResponse};
use serde::{Deserialize};

#[allow(clippy::unused_async)]
pub async fn to_sarcastic_text(
    to_sarcastic_text_request: web::Form<ToSarcasticTextRequest>,
) -> Result<impl Responder, Error> {

    let sarcastic_text: String = to_sarcastic_text_request.original_text.chars().map(|c| match c {
        'a'..='z' => (c as u8 - u8::from(rand::random::<bool>()) * 32u8).into(),
        'A'..='Z' => (c as u8 + u8::from(rand::random::<bool>()) * 32u8).into(),
        _ => c
    }).collect();

    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(sarcastic_text))
}

#[derive(Deserialize)]
pub struct ToSarcasticTextRequest {
    original_text: String,
}
