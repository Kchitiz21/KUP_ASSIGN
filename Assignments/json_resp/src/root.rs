extern crate reqwest;
extern crate tokio;

///json:json function is used to get the json response from the url
///
/// #Arguments
///
/// No Arguments
///
/// Return
///
/// Return Result<(), reqwest::Error>
#[tokio::main]
pub async fn json() -> Result<(), reqwest::Error> {
    env_logger::init();
    let url: String = String::from("https://pokeapi.co/api/v2/pokemon-species/ditto");
    let json: serde_json::Value = reqwest::Client::new()
        .get(url).send().await?.json()
        .await?;

    log::info!("{:?}", json.get("name"));
    Ok(())
}
