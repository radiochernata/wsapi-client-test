use tera::{Tera, Context};
use std::path::Path;
use serde::{Serialize, Deserialize, Serializer, ser::SerializeStruct};

#[derive(Debug, Serialize, Deserialize)]
struct MyPage
{
    my_title: String,
    my_body: String,
}

#[derive(Debug, Deserialize, Clone)]
struct MyAddress
{
    my_first_name: String,
    my_second_name: String,
    my_city: String,
    my_street: String,
    my_house: String,
    my_post_code: String,
}

impl Serialize for MyAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let name = format!("{} {}", self.my_first_name, self.my_second_name);
        let address = format!("{} {}, {} {}", self.my_post_code, self.my_city, self.my_street, self.my_house);

        let mut obj = serializer.serialize_struct("MyAddress", 2)?;
        obj.serialize_field("name", &name)?;
        obj.serialize_field("address", &address)?;
        obj.end()
    }
}

fn json_page(page: MyPage) -> serde_json::Value {
    serde_json::json!({
        "title": page.my_title,
        "body": page.my_body
    })
}

fn json_address(address: Vec<MyAddress>) -> serde_json::Value {
    let json_obj = serde_json::to_value(address).unwrap();
    serde_json::json!({
        "label": "My address book",
        "addresses": json_obj
    })
}

fn use_template_page(page: MyPage) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    tera.add_template_file(Path::new("page.json"), None)?;
    let rendered = tera.render("page.json", &Context::from_serialize(json_page(page))?)?;
    Ok(rendered)
}

fn use_template_address(address: Vec<MyAddress>) -> Result<String, tera::Error> {
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
    let my_page: MyPage = MyPage{my_title: String::from("My Page"), 
                            my_body: String::from("Welcome to my page!")};
    let address_brother: MyAddress = MyAddress{my_first_name: String::from("Sherlock"),
                                my_second_name: String::from("Holmes"),
                                my_city: String::from("London"),
                                my_street: String::from("Baker street"),
                                my_house: String::from("221B"),
                                my_post_code: String::from("NW1 6XE")};
    let address_sister: MyAddress = MyAddress{my_first_name: String::from("Sharlin"),
                                my_second_name: String::from("Holmes"),
                                my_city: String::from("London"),
                                my_street: String::from("Baker street"),
                                my_house: String::from("221B"),
                                my_post_code: String::from("NW1 6XE")};
    
    let mut my_address_book: Vec<MyAddress> = Vec::new();
    my_address_book.push(address_brother);
    my_address_book.push(address_sister);

    println!("{}", use_template_page(my_page)?);
    println!("{}", json_address(my_address_book.clone()));
    println!("{}", use_template_address(my_address_book)?);

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
