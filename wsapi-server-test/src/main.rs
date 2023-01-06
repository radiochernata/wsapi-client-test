use actix_web::{get, web, post, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/label/{name}")]
async fn get_label(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[post("/label")]
async fn create_label(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_label)
            .service(create_label)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}