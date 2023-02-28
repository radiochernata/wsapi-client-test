//use std::io::{stdout, Write};
use std::fs::File;
//use curl::easy::Easy;
use serde::Deserialize;
use hyper::client::Client;
use hyper::body::HttpBody as _;
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
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

async fn get_label(url: &str, labelname: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let uri = format!("{}{}", url, labelname).parse()?;

    // Await the response...
    let mut resp = client.get(uri).await?;

    println!("Response: {}", resp.status());
    match resp.status() {
        hyper::StatusCode::OK => {
            while let Some(chunk) = resp.body_mut().data().await {
                stdout().write_all(&chunk?).await?;
            }
        },
        _ => println!("Not OK"),
    }

    Ok(())
}