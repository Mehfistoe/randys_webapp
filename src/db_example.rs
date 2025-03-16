use actix_web::{web, App, HttpResponse, Responder};
use diesel::prelude::*;


#[derive(Debug, Queryable)]
struct User {
    id: i32,
    name: String,
}



// This code connects to a PostgreSQL database and creates a new App that responds to GET requests to the /users URL with a JSON response containing all users in the database.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to the database
    let conn = diesel::r2d2::ConnectionManager::new(diesel::r2d2::Pool::new(
        diesel::r2d2::ConnectionManager::Postgres::new("host=localhost user=myuser password=mypassword dbname=mydb"),
    ));

    // Create a new App
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .route("/users", web::get().to(get_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_users(conn: web::Data<diesel::r2d2::ConnectionManager<diesel::r2d2::Pool>>) -> impl Responder {
    // Get all users from the database
    let users = diesel::r2d2::ConnectionManager::new(conn).get_result::<User>(&[diesel::r2d2::ConnectionManager::new(conn).get()])?;

    // Return the users as a JSON response
    HttpResponse::Ok().json(users)
}