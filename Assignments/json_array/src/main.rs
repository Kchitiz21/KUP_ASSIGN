extern crate reqwest;
extern crate tokio;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Welcome {
    color: Color,
    egg_groups: Vec<Color>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    name: String,
    url: String,
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
    let res = reqwest::get("https://pokeapi.co/api/v2/pokemon-species/ditto")
        .await?
        .json::<Welcome>()
        .await?;
    log::info!("First array: {:?}", res.egg_groups[0]);
    Ok(())
}
