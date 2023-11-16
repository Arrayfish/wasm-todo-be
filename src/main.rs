use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


async fn get_todos() -> impl Responder {
    // get user id from the session
    
    // access to the database
    HttpResponse::Ok().body("Hey there!")
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
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/todo")
                    .route("/todos", web::get().to(get_todos))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}