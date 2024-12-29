extern crate reqwest;
use reqwest::header;

const API_ENDPOINT: &str = "https://apicollections.parismusees.paris.fr/graphql";

fn main() {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("auth-token", "API_KEY".parse().unwrap());

    let client = reqwest::blocking::Client::builder().build().unwrap();

    let res = client.post(API_ENDPOINT).headers(headers).body("{\"query\": \"{  nodeQuery(     filter: {}, offset: 3, limit: 1   ) {     entities {       entityUuid     }   } }\"}")
        .send().unwrap()
        .text().unwrap();

    println!("{}", res);
}
