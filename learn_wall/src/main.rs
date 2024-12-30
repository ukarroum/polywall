extern crate reqwest;
use reqwest::header;

const API_ENDPOINT: &str = "https://apicollections.parismusees.paris.fr/graphql";

fn main() {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("auth-token", "API_KEY".parse().unwrap());

    let client = reqwest::blocking::Client::builder().build().unwrap();

    let res = client.post(API_ENDPOINT).headers(headers.clone()).body("{\"query\": \"{  nodeQuery(     filter: {conditions: [{field: \\\"field_oeuvre_types_objet.entity.field_lref_adlib\\\", value: \\\"4284\\\"}]}, offset: 3, limit: 1   ) {     entities {       entityUuid     }   } }\"}")
        .send().unwrap()
        .text().unwrap();

    println!("{}", res);

    let start = "{\"data\":{\"nodeQuery\":{\"entities\":[{\"entityUuid\":\"".len();
    let uuid = &res[start..start + 36];

    let query = format!("{{\"query\": \"{{   nodeQuery(     filter: {{conditions: [{{field: \\\"uuid\\\", value: \\\"{uuid}\\\"}}]}}   ) {{     entities {{       entityUuid       ... on NodeOeuvre {{         title         fieldVisuels {{           entity {{             publicUrl           }}         }}       }}     }}   }} }}\"}}");
    println!("{}", query);

    let res = client.post(API_ENDPOINT).headers(headers.clone()).body(query)
        .send().unwrap()
        .text().unwrap();


    println!("{}", res);
}
