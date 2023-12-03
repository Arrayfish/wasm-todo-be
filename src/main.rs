use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use sea_orm::{Database, EntityTrait, FromQueryResult, IntoActiveModel, QueryFilter, QueryOrder, Set, DatabaseConnection, ActiveModelTrait};
use entity::{prelude::*, *};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize)]
pub struct AllTodoLists {
    pub todo_lists: Vec<todo_list::Model>,
    pub todos: Vec<todo::Model>,
}
async fn get_all_todolists_and_todos(data: web::Data<AppState>) -> Result<impl Responder> {
    // TODO: get user id from the session
    let db = &data.db;
    let all_todo_lists: Vec<todo_list::Model> = TodoList::find().all(db).await.unwrap();
    let all_todos: Vec<todo::Model> = Todo::find().all(db).await.unwrap();
    // access to the database
    Ok(web::Json(AllTodoLists {
        todo_lists: all_todo_lists,
        todos: all_todos,
    }))
}
async fn create_todo(data: web::Data<AppState>, todo: web::Json<todo::Model>) -> impl Responder {
    // access to the database
    let db = &data.db;
    let todo: todo::ActiveModel = todo.into_inner().into();
    let res = todo.save(db).await;
    match res{
        Ok(_) => HttpResponse::Ok().body("Todo created!"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating todo!"),
    }
}

async fn update_todo(todo: web::Json<todo::Model>) -> impl Responder {
    // access to the database
    HttpResponse::Ok().body("Hey there!")
}

async fn delete_todo(todo: web::Json<todo::Model>) -> impl Responder {
    // access to the database
    HttpResponse::Ok().body("Hey there!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgres://postgres:password@localhost:5436/todo_db";
    let db = Database::connect(database_url).await.unwrap();
    let app_state = AppState { db: db };
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(app_state.clone()))
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