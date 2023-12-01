use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{Database, EntityTrait, FromQueryResult, IntoActiveModel, QueryFilter, QueryOrder, Set, DatabaseConnection};
use entity::{prelude::*, *};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn get_all_todolists_and_todos(db: web::Data<DatabaseConnection>) -> impl Responder {
    // get user id from the session
    let todo_lists: Vec<todo_list::Model> = TodoList::all(db.get_ref()).await.unwrap();
    // access to the database
    HttpResponse::Ok().body(todo_lists)
}

async fn create_todo(req_body: String) -> impl Responder {
    // access to the database
    HttpResponse::Ok().body("Hey there!")
}

async fn update_todo(id: u32, req_body: String) -> impl Responder {
    // access to the database
    HttpResponse::Ok().body("Hey there!")
}

async fn delete_todo() -> impl Responder {
    // access to the database
    HttpResponse::Ok().body("Hey there!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgres://postgres:password@localhost:5436/todo_db";
    let db = Database::connect(database_url).await.unwrap();
    let app_state = AppState { db: db.clone() };
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(app_state))
            .service(
                web::scope("/todo")
                    .route("/todos", web::get().to(get_todos))
            )
            .route("/", web::get().to(get_all_todolists_and_todos))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}