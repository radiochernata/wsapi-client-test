use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result, get, post};
use serde::{Serialize, Deserialize};
use std::fs::File;

#[derive(Serialize)]
struct MyObj {
    service: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Route {
    url: String,
    service: String,
    method: String,
}

#[derive(Deserialize, Debug)]
struct Config{
    host: String,
    port: u16,
    routes: Vec<Route>,
}

async fn get_postlabel(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        service: "postlabel".to_string(),
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

async fn get_dhllabel(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        service: "dhllabel".to_string(),
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

async fn create_label(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_open_result = File::open("config.yml").expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(config_open_result).unwrap();

    HttpServer::new(|| {
        App::new()
            
            .service(web::scope("/label")
                .route("/post/{name}", web::get().to(get_postlabel))
                .route("/dhl/{name}", web::get().to(get_dhllabel))
                .route("", web::post().to(create_label))
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
