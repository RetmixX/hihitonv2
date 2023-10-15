mod utils;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::utils::documentation::SwaggerDoc;
use crate::utils::responses::MessageResponse;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Start server");
    run().await
}

#[utoipa::path(
    get,
    path = "/api/healthcheck",
    responses(
        (
            status = 200,
            body = MessageResponse
        )
    )
)]
#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json(MessageResponse::new("Ok"))
}


async fn run() -> std::io::Result<()> {
    let swagger = SwaggerDoc::openapi();
    HttpServer::new(move || {
        App::new()
            .default_service(web::route().to(not_found))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", swagger.clone())
            )
            .service(web::scope("/api")
                .service(healthcheck))
    }).bind(("0.0.0.0", 8000))?.run().await
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not found")
}
