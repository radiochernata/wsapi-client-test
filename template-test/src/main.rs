use tera::{Tera, Context};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyPage
{
    my_title: String,
    my_body: String,
}

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
    println!("{}", json_address(my_address));
    Ok(())
}
