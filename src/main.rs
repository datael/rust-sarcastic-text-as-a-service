mod api;

use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder, get};
use api::to_sarcastic_text;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(index)
            .route("/", web::post().to(to_sarcastic_text))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.htm")))
}
