extern crate reqwest;
use reqwest::header;
use regex::Regex;
use std::process::Command;
use std::env;
use std::string::String;
use rand::Rng;

const API_ENDPOINT: &str = "https://apicollections.parismusees.paris.fr/graphql";

fn main() {
    let api_key: String = env::var("PARIS_MUSEE_API_KEY").unwrap();

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("auth-token", api_key.parse().unwrap());

    let client = reqwest::blocking::Client::builder().build().unwrap();

    // 4284 => Paiting
    let res = client.post(API_ENDPOINT).headers(headers.clone()).body(r#"{"query": "{ nodeQuery( filter: {conditions: [{field: \"field_oeuvre_types_objet.entity.field_lref_adlib\", value: \"4284\"}, {field: \"field_visuels.entity.field_image_libre\", value: \"1\"}]}) { count }}"}"#)
        .send().unwrap()
        .text().unwrap();

    let re = Regex::new("count\":(?<count>.*?)}").unwrap();

    let Some(json) = re.captures(&res) else {
        println!("No match");
        return;
    };

    let count: i64 = json["count"].parse().unwrap();


    let mut rng = rand::rng();
    let offset = rng.random_range(0..count);

    let res = client.post(API_ENDPOINT).headers(headers.clone()).body(r#"{"query": "{ nodeQuery( filter: {conditions: [{field: \"field_oeuvre_types_objet.entity.field_lref_adlib\", value: \"4284\"}, {field: \"field_visuels.entity.field_image_libre\", value: \"1\"}]}, offset: {offset}, limit: 1) { entities { ... on NodeOeuvre { title fieldVisuels { entity {publicUrl}} }}}}"}"#.replace(r"{offset}", &offset.to_string()))
        .send().unwrap()
        .text().unwrap();

    let re = Regex::new("publicUrl\":\"(?<url>.*?)\"").unwrap();

    let Some(json) = re.captures(&res) else {
        println!("No match");
        return;
    };

    let url = json["url"].replace("\\", "");

    Command::new("gsettings").args(["set", "org.gnome.desktop.background", "picture-uri", &url]).spawn().unwrap();
    Command::new("gsettings").args(["set", "org.gnome.desktop.background", "picture-options", "scaled"]).spawn().unwrap();
    Command::new("gsettings").args(["set", "org.gnome.desktop.background", "primary-color", "000000"]).spawn().unwrap();
}
