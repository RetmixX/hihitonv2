use actix_web::{App, HttpResponse, HttpServer, Responder, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("ALLO");
    run().await
}

async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Work!")
}



async fn run() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .default_service(web::route().to(not_found))
            .service(web::scope("/api")
                .route("/healthcheck", web::get().to(healthcheck))
            )
    }).bind(("0.0.0.0", 8000))?.run().await
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not found")
}
