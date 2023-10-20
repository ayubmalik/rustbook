use reqwest::{Client, Error};
use serde_json::Value;

const YTS_URL: &str = "https://yts.mx/api/v2/list_movies.json?limit=1";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = get_movies().await?;
    println!("movies = {:?}", response["data"]["movies"]);
    Ok(())
}

async fn get_movies() -> Result<Value, Error> {
    let client = Client::new();

    let response = client.get(YTS_URL).send().await?.text().await?;
    let value: Value = match serde_json::from_str(&response) {
        Ok(v) => v,
        Err(_) => panic!("f"),
    };

    Ok(value)
}
