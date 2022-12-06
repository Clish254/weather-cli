use reqwest;
use serde_json;

fn main() {
    let api_token = std::env::var("API_TOKEN").expect("expected there to be an api token");
    println!("{}", api_token);

    let mut arg_iterator = std::env::args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();

    dbg!(&args);

    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", api_token), ("keyword", args)])
        .send()
        .expect("a successful request")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");
    println!("{:#?}", response)
}
