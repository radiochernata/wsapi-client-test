use tera::{Tera, Context};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyPage
{
    my_title: String,
    my_body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MyAddress
{
    my_first_name: String,
    my_second_name: String,
    my_city: String,
    my_street: String,
    my_house: String,
    my_post_code: String,
}

fn json_page(page: MyPage) -> serde_json::Value {
    serde_json::json!({
        "title": page.my_title,
        "body": page.my_body
    })
}

fn json_address(address: MyAddress) -> serde_json::Value {
    serde_json::json!({
        "name": format!("{} {}", address.my_first_name, address.my_second_name),
        "address": format!("{} {}, {} {}", address.my_post_code, address.my_city, address.my_street, address.my_house)
    })
}

fn use_template_page(page: MyPage) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    tera.add_template_file(Path::new("page.json"), None)?;
    let rendered = tera.render("page.json", &Context::from_serialize(json_page(page))?)?;
    Ok(rendered)
}

fn use_template_address(address: MyAddress) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    tera.add_template_file(Path::new("address.json"), None)?;
    let rendered = tera.render("address.json", &Context::from_serialize(json_address(address))?)?;
    Ok(rendered)
}

fn use_template_businesscard(businesscard: serde_json::Value) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    tera.add_template_file(Path::new("businesscard.json"), None)?;
    let rendered = tera.render("businesscard.json", &Context::from_serialize(businesscard)?)?;
    Ok(rendered)
}

fn main() -> tera::Result<()> {
    let my_page = MyPage{my_title: String::from("My Page"), 
                         my_body: String::from("Welcome to my page!")};
    let my_address = MyAddress{my_first_name: String::from("Sherlock"),
                               my_second_name: String::from("Holmes"),
                               my_city: String::from("London"),
                               my_street: String::from("Baker street"),
                               my_house: String::from("221B"),
                               my_post_code: String::from("NW1 6XE")};

    println!("{}", use_template_page(my_page)?);
    println!("{}", json_address(my_address.clone()));
    println!("{}", use_template_address(my_address)?);

    // use template for empty field
    println!("");
    println!("Tests with empty field");
    let his_businesscard = serde_json::json!({
        "name": String::from("John Joy"),
        "phone": String::from("+0123456789"),
        "email": String::from("johnjoy@example.com"),
        "fax": String::from("+0123456788")
    });

    let her_businesscard = serde_json::json!({
        "name": String::from("Taja Joy"),
        "phone": String::from("+0123456777"),
        "email": String::from("tajajoy@example.com"),
    });

    println!("Test with fax field");
    println!("{}", use_template_businesscard(his_businesscard)?);
    println!("Test without fax field");
    println!("{}", use_template_businesscard(her_businesscard)?);
    Ok(())
}
