use actix_web::{web, App, HttpResponse, Responder};


async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}!", name)
}

async fn goodbye() -> Responder {
    HttpResponse::Ok().body("Goodbye!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // route for the /hello URL with a name parameter
            .route("/hello/{name}", web::get().to(hello))
            // route for the /goodbye URL
            .route("/goodbye", web::get().to(goodbye))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}