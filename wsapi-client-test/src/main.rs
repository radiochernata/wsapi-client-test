 //use std::io::{stdout, Write};
use std::fs::File;
use serde::Deserialize;
//use hyper::client::Client;
//use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

#[derive(Deserialize, Debug)]
struct Service{
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    services: Vec<Service>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let config_open_result = File::open("config.yml").expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(config_open_result).unwrap();

    println!("{:?}", config);

    for service in config.services {
        match service.name.as_str() {
            "dhl" => get_label(service.url.as_str(), "nata").await?,
            "post" => get_label(service.url.as_str(), "tatiana").await?,
            _ => println!("Unknown service"),
        }
    }
    Ok(())
}

async fn get_label(url: &str, labelname: &str) -> Result<(), reqwest::Error> {
    let uri = format!("{}{}", url, labelname);

    // Await the response...
    let mut resp = reqwest::get(uri).await?;

    println!("Response: {}", resp.status());
    match resp.status() {
        reqwest::StatusCode::OK => {
            let body = resp.text().await?;
            println!("{}", body)
            },
        _ => println!("Not OK"),
    }

    Ok(())
}