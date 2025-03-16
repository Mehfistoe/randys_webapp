use actix_web::{web, App, HttpServer, Responder}; // importing crates(modules) and the items and functions within
/*
In the actix web framework:
web: Module. Essential help functions and types for app registration
App: Struct. The top-level builder for an Actix Web app. Registers routes for resources and middleware and stores application state
    shared across all handlers withing the same URL path prefix.
HttpServer: Struct. An HTTP server
Responder: Trait (interface that data types can implement). Trait implemented by types that can be converted to an HTTP response
*/

// This code creates a basic web application that responds to GET requests to the root URL with the string “Hello, World!”.

async fn index() -> impl Responder {
    "Hello world!" // HTML page would go here?

}


#[actix_web::main]
/*
This is an attibute macro. Attribute macros in Rust are a form of metaprogramming that allows you to add metadata
or modify the behavior of code. #[actix_web::main] specifically tell the Actix Web runtime to set up the necessary
environment for running an async web server.
The macro is essential for running an Actix web server and ensures the async main function is executed in the
correct context. It is used to setup the Actix Web system entry-point, and is needed for actor support within the
actix web. I realize this whole paragraph is redeundant lol.
 */

// std::io is a module ahta contains a number of common things you'll need when doing input and output
// io::Result is a specialized type for I/O operations and is used for any operation which may produce an error
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {  // HttpServer constructs an application instance
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("10.0.1.9:8080")?  // Resolves socket address(es) and binds server to created listener(s)
    .run()  // start listening for incoming connections
    .await  // suspends the execution of the main function until the awaited future produces a value
}