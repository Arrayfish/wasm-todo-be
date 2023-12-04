use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use sea_orm::{Database, EntityTrait,  DatabaseConnection, ActiveModelTrait, ActiveValue};
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

/**
 * Create a todo
 * @param {todo::Model} todo
 * @returns {todo::Model} todo
 * @throws {Error} error
 */
async fn create_todo(data: web::Data<AppState>, todo: web::Json<todo::Model>) -> impl Responder {
    // access to the database
    let db = &data.db;
    let mut todo: todo::ActiveModel = todo.into_inner().into();
    todo.id = ActiveValue::NotSet;
    let res = todo.save(db).await;
    match res{
        Ok(_) => HttpResponse::Ok().body(format!("Todo created! id: {:?}",res.unwrap().id.unwrap())),
        Err(err) => {
            println!("Error: {}",err);
            HttpResponse::InternalServerError().body("Error creating todo!")
        }
    }
}

async fn update_todo(data: web::Data<AppState>, todo: web::Json<todo::Model>) -> impl Responder {
    // access to the database
    let db = &data.db;
    let todo: todo::ActiveModel = todo.into_inner().into();
    println!("Todo: {:?}", &todo);
    let todo = todo.reset_all();
    let res = todo.update(db).await;
    match res{
        Ok(res) => {
            println!("Todo updated: {:?}",res);
            HttpResponse::Ok().body("Todo updated!")
        }
        Err(err) => {
            println!("Error: {}",err);
            HttpResponse::InternalServerError().body("Error update todo!")
        }
    }
}

async fn delete_todo(data: web::Data<AppState>, todo: web::Json<todo::Model>) -> impl Responder {
    // access to the database
    let db = &data.db;
    let res = Todo::delete_by_id(todo.id).exec(db).await;
    match res{
        Ok(_) => HttpResponse::Ok().body("Todo deleted!"),
        Err(_) => HttpResponse::InternalServerError().body("Error delete todo!"),
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgres://postgres:password@localhost:5436/todo_db";
    let db = Database::connect(database_url).await.unwrap();
    let app_state = AppState { db: db };
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(get_all_todolists_and_todos))
            .route("/create", web::post().to(create_todo))
            .route("/update", web::post().to(update_todo))
            .route("/delete", web::post().to(delete_todo))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}