//we use the attribute to ensure it’s executed with the actix runtime
use actix_web::{web, App, HttpServer};
use controllers::*;
use std::sync::*;
//use MongoDB::{options::ClientOptions, Client};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    //Creating HttpServer and App instance with few routes that
    //would point to controllers module which would handle the logic of each route
    //and serve it on port 8080
    let mut client_options = ClientOptions::parse("MongoDB://127.0.0.1:27017/todolist")
        .await
        .unwrap();
    client_options.app_name = Some("Todolist".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    HttpServer::new(move || {
        App::new()
            .route("/todos", web::get().to(controllers::get_todos))
            .route("/todos", web::post().to(controllers::create_todo))
            .route("/todos/{id}", web::get().to(controllers::fetch_one))
            .route("/todos/{id}", web::patch().to(controllers::update_todo))
            .route("/todos/{id}", web::delete().to(controllers::delete_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
