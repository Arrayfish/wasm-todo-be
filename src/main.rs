mod login_check;

use actix_web::{
    cookie::Key,
    dev, get,
    http::{header, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers, DefaultHeaders},
    post, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use entity::{prelude::*, *};
use sea_orm::{
ActiveModelTrait, ActiveValue, ColumnTrait, Database, EntityTrait,
    QueryFilter,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use password_auth;

use login_check::CheckLogin;
mod app;
use app::AppState;
mod todo;
use todo::{
    create_todo,
    delete_todo,
    update_todo,
    get_all_todolists_and_todos,
    create_todo_list,
    update_todo_list,
    delete_todo_list,
};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}
async fn register(
    data: web::Data<AppState>,
    request: HttpRequest,
    json: web::Json<LoginForm>,
) -> impl Responder {
    // password hashの作成
    let password_hash = password_auth::generate_hash(&json.password);
    // ユーザの保存
    let db = &data.db;
    let user: user::ActiveModel = user::ActiveModel {
        id: ActiveValue::NotSet,
        email: ActiveValue::Set(json.email.clone()),
        password: ActiveValue::Set(password_hash),
    };
    match User::insert(user).exec(db).await {
        Ok(inserted_user) => {
            println!("User created with ID: {}", inserted_user.last_insert_id);
            Identity::login(&request.extensions(), inserted_user.last_insert_id.to_string()).unwrap();
        }
        Err(err) => {
            println!("Error: {}", err);
            return HttpResponse::InternalServerError().body("Error creating user!");
        }
    }
    HttpResponse::Ok().body("Login")
}
async fn login(
    data: web::Data<AppState>,
    request: HttpRequest,
    json: web::Json<LoginForm>,
) -> impl Responder {
    // ユーザの認証
    let db = &data.db;
    let user_option: Option<user::Model> = User::find()
        .filter(user::Column::Email.eq(json.email.clone()))
        .one(db)
        .await
        .unwrap();
    if let Some(user) = user_option {
        match password_auth::verify_password(&json.password, &user.password) {
            Ok(_) => {
                println!("Password is ok");
                Identity::login(&request.extensions(), user.id.to_string()).unwrap();
            }
            Err(_) => return HttpResponse::Unauthorized().body("Invalid email or password"),
        }
    } else {
        return HttpResponse::Unauthorized().body("Invalid email or password");
    }
    HttpResponse::Ok().body("Login")
}

async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok().body("Logout")
}


fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgres://postgres:password@localhost:5436/todo_db";
    let db = Database::connect(database_url).await.unwrap();
    let app_state = AppState { db: db };
    let secret_key = Key::generate();
    HttpServer::new(move || {
        App::new()
            .wrap(DefaultHeaders::new()
                .add(("Strict-Transport-Security", "max-age=63072000; includeSubDomains"))
                .add(("Content-Security-Policy", "default-src 'self'"))
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-Frame-Options", "SAMEORIGIN"))
                .add(("X-XSS-Protection", "1; mode=block"))
                .add(("Access-Control-Allow-Origin", "*"))
            )
            .app_data(web::Data::new(app_state.clone()))
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
            .wrap(CheckLogin::new(vec![
                "/auth/login".to_string(),
                "/auth/register".to_string(),
            ]))
            .wrap(
                IdentityMiddleware::builder()
                    .visit_deadline(Some(Duration::new(30 * 60, 0))) // 30 minutes
                    .login_deadline(Some(Duration::new(24 * 60 * 60, 0))) // 24 hours
                    .build(),
            )
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(login))
                    .route("/logout", web::get().to(logout))
                    .route("/register", web::post().to(register)),
            )
            .route("/", web::get().to(get_all_todolists_and_todos))
            .service(
                web::scope("/todo")
                    .route("/create", web::post().to(create_todo))
                    .route("/update", web::post().to(update_todo))
                    .route("/delete", web::post().to(delete_todo)),
            )
            .service(
                web::scope("/todo_list")
                    .route("/create", web::post().to(create_todo_list))
                    .route("/update", web::post().to(update_todo_list))
                    .route("/delete", web::post().to(delete_todo_list)),
            )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
