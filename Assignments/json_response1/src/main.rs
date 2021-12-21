extern crate reqwest;
extern crate tokio;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Welcome {
    #[serde(rename = "_id")]
    id: i64,
    name: Name,
    contribs: Vec<String>,
    awards: Vec<Award>,
}

#[derive(Serialize, Deserialize)]
pub struct Award {
    award: String,
    year: i64,
    by: String,
}

#[derive(Serialize, Deserialize)]
pub struct Name {
    first: String,
    last: String,
}
///main: main function is used to get the URL
///
/// #Arguments
///
/// No Arguments
///
/// Return
///
/// Return Result<(), reqwest::Error>

#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    let res = reqwest::get("https://mocki.io/v1/e90a6cb0-9a59-4080-b7ab-4739c2b1483f")
        .await?
        .json::<Welcome>()
        .await?;
    log::info!("Name: {:#?}", res.name.first);
    Ok(())
}
