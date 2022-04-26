use actix_web::Responder;

//Handlers for each route
//they are each asynchronous functions that r
//return a Responder trait provided by actix-web

pub async fn get_todos() -> impl Responder {
    format!("fetch all todos");
}

pub async fn create_todo() -> impl Responder {
    format!("Creating a new todo item");
}

pub async fn fetch_one() -> impl Responder {
    format!("Fetch one todo item");
}

pub async fn update_todo() -> impl Responder {
    format!("Update a todo item");
}
pub async fn delete_todo() -> impl Responder {
    format!("Delete a todo item");
}
