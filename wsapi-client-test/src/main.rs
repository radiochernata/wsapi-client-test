use std::io::{stdout, Write};
use std::fs::File;
//use curl::easy::Easy;
use serde::Deserialize;
use hyper::body;
use hyper::{client, Uri};

#[derive(Deserialize, Debug)]
struct Service{
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    services: Vec<Service>,
}

fn main() {
    let config_open_result = File::open("config.yml").expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(config_open_result).unwrap();

    println!("{:?}", config);

    for service in config.services {
        match service.name.as_str() {
            "dhl" => get_dhllabel(service.url.as_str(), "nata"),
            "post" => get_postlabel(service.url.as_str(), "tatiana"),
            _ => println!("Unknown service"),
        }
    }

}

fn get_dhllabel(url: &str, labelname: &str) {
    // let mut easy = Easy::new();
    // easy.url(format!("{}{}", url, labelname).as_str()).unwrap();
    // easy.write_function(|data| {
    //     stdout().write_all(data).unwrap();
    //     Ok(data.len())
    // }).unwrap();
    // easy.perform().unwrap();

    let client = client::new();

    let res = client.get(Uri::from_static(format!("{}{}", url, labelname).as_str())).unwrap();

    // And then, if the request gets a response...
    println!("status: {}", res.status());

    // Concatenate the body stream into a single buffer...
    let buf = body::to_bytes(res).unwrap();

    println!("body: {:?}", buf);
}

fn get_postlabel(url: &str, labelname: &str) {
    // let mut easy = Easy::new();
    // easy.url(format!("{}{}", url, labelname).as_str()).unwrap();
    // easy.write_function(|data| {
    //     stdout().write_all(data).unwrap();
    //     Ok(data.len())
    // }).unwrap();
    // easy.perform().unwrap();
}