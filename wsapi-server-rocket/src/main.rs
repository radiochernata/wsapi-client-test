use serde::{Serialize, Deserialize};
use std::fs::File;
use rocket::get;

#[derive(Deserialize, Debug, Clone)]
struct Route {
    url: String,
    service: String,
    method: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Config{
    host: String,
    port: u16,
    routes: Vec<Route>,
}

#[get("/<name>")]
fn index(name: String) -> String {
    format!("Hello, {}!", name)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error>  {
    let config_open_result = File::open("config.yml").expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(config_open_result).unwrap();

    let mut app = rocket::build();
    for route in &config.routes {
        app = app.mount(route.url.as_str(), rocket::routes![index]);
            //.mount("/dhl/", routes![index])
    } 
    app.launch().await?;   

    Ok(())
}