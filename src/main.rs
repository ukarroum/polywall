extern crate reqwest;
use reqwest::header;

const API_ENDPOINT: &str = "https://apicollections.parismusees.paris.fr/graphql";

fn main() {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("auth-token", "c4a91329-0317-42b0-94c5-9fa91b1360c7".parse().unwrap());

    let client = reqwest::blocking::Client::builder().build().unwrap();

    // 4284 => 
    let res = client.post(API_ENDPOINT).headers(headers.clone()).body(r#"{"query": "{  nodeQuery(     filter: {conditions: [{field: \"field_oeuvre_types_objet.entity.field_lref_adlib\", value: \"4284\"}]}, offset: 3, limit: 1   ) {     entities {       entityUuid ... on NodeOeuvre { title fieldVisuels { entity {publicUrl}} }     }   } }"}"#)
        .send().unwrap()
        .text().unwrap();

    println!("{}", res);
}
