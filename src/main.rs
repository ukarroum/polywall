extern crate reqwest;
use reqwest::header;
use regex::Regex;

const API_ENDPOINT: &str = "https://apicollections.parismusees.paris.fr/graphql";

fn main() {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("auth-token", "API_KEY".parse().unwrap());

    let client = reqwest::blocking::Client::builder().build().unwrap();

    // 4284 => Paiting
    let res = client.post(API_ENDPOINT).headers(headers.clone()).body(r#"{"query": "{ nodeQuery( filter: {conditions: [{field: \"field_oeuvre_types_objet.entity.field_lref_adlib\", value: \"4284\"}, {field: \"field_visuels.entity.field_image_libre\", value: \"1\"}]}, offset: 3, limit: 1) { entities { ... on NodeOeuvre { title fieldVisuels { entity {publicUrl}} }}}}"}"#)
        .send().unwrap()
        .text().unwrap();

    println!("{}", res);

    let re = Regex::new("publicUrl\":\"(?<url>.*)\"").unwrap();

    let Some(json) = re.captures(&res) else {
        println!("No match");
        return;
    };

    let url = json["url"].replace("\\", "");
    println!("Url: {}", &url);
}
