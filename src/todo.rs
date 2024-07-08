use serde::Serialize;
use actix_web::{web, HttpResponse, Responder, Result, get, post, delete, put};
use actix_identity::Identity;
use sea_orm::{LoaderTrait, ActiveValue};
use entity::{prelude::*, *};
use crate::app::AppState;


#[derive(Debug, Clone, Serialize)]
pub struct AllTodoLists {
    pub todo_lists: Vec<todo_list::Model>,
    pub todos: Vec<Vec<todo::Model>>,
}
/**
 * Get all todo lists and todos
 * @returns {AllTodoLists} AllTodoLists
 * @throws {Error} error
 */
pub async fn get_all_todolists_and_todos(
    data: web::Data<AppState>,
    user: Option<Identity>,
) -> Result<impl Responder> {
    let user_id : String;
    if let Some(user) = user {
        user_id = user.id().unwrap();
    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }

    
    let db = &data.db;
    let todo_lists: Vec<todo_list::Model> = TodoList::find().filter(todo_list::Column::UserId.eq(user_id)).all(db).await.unwrap();
    let all_todos: Vec<Vec<todo::Model>> = todo_lists.load_many(Todo, db).await.unwrap();
    // access to the database
    Ok(HttpResponse::Ok().json(AllTodoLists {
        todo_lists: todo_lists,
        todos: all_todos,
    }))
}

/**
 * Create a todo
 * @param {todo::Model} todo
 * @returns {todo::Model} todo
 * @throws {Error} error
 */
pub async fn create_todo(data: web::Data<AppState>, todo: web::Json<todo::Model>) -> impl Responder {
    // access to the database
    let db = &data.db;
    let mut todo: todo::ActiveModel = todo.into_inner().into();
    todo.id = ActiveValue::NotSet;
    let res = todo.save(db).await;
    match res {
        Ok(_) => {
            HttpResponse::Ok().body(format!("Todo created! id: {:?}", res.unwrap().id.unwrap()))
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error creating todo!")
        }
    }
}

pub async fn update_todo(data: web::Data<AppState>, todo: web::Json<todo::Model>) -> impl Responder {
    // access to the database
    let db = &data.db;
    let todo: todo::ActiveModel = todo.into_inner().into();
    println!("Todo: {:?}", &todo);
    let todo = todo.reset_all();
    let res = todo.update(db).await;
    match res {
        Ok(res) => {
            println!("Todo updated: {:?}", res);
            HttpResponse::Ok().body("Todo updated!")
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error update todo!")
        }
    }
}

pub async fn delete_todo(data: web::Data<AppState>, todo: web::Json<todo::Model>) -> impl Responder {
    // access to the database
    let db = &data.db;
    let res = Todo::delete_by_id(todo.id).exec(db).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("Todo deleted!"),
        Err(_) => HttpResponse::InternalServerError().body("Error delete todo!"),
    }
}

pub async fn create_todo_list(
    data: web::Data<AppState>,
    todo_list: web::Json<todo_list::Model>,
    user: Option<Identity>,
) -> impl Responder {
    // access to the database
    let db = &data.db;
    let mut todo_list: todo_list::ActiveModel = todo_list.into_inner().into();
    todo_list.id = ActiveValue::NotSet;
    if let Some(user) = user {
        todo_list.user_id = ActiveValue::Set(user.id().unwrap().parse().unwrap());
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
    let res = todo_list.save(db).await;
    match res {
        Ok(_) => HttpResponse::Ok().body(format!(
            "Todo list created! id: {:?}",
            res.unwrap().id.unwrap()
        )),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error creating todo list!")
        }
    }
}

pub async fn update_todo_list(
    data: web::Data<AppState>,
    todo_list: web::Json<todo_list::Model>,
) -> impl Responder {
    // access to the database
    let db = &data.db;
    let todo_list: todo_list::ActiveModel = todo_list.into_inner().into();
    println!("Todo list: {:?}", &todo_list);
    let todo_list = todo_list.reset_all();
    let res = todo_list.update(db).await;
    match res {
        Ok(res) => {
            println!("Todo list updated: {:?}", res);
            HttpResponse::Ok().body("Todo list updated!")
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error update todo list!")
        }
    }
}

pub async fn delete_todo_list(
    data: web::Data<AppState>,
    todo_list: web::Json<todo_list::Model>,
) -> impl Responder {
    // access to the database
    let db = &data.db;
    let res = TodoList::delete_by_id(todo_list.id).exec(db).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("Todo list deleted!"),
        Err(err) => {
            let error_message = format!("Error deleting todo list: {}", err);
            HttpResponse::InternalServerError().body(error_message)
        }
    }
}