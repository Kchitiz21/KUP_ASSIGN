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
    let json: serde_json::Value = reqwest::Client::new().get("https://pokeapi.co/api/v2/pokemon-species/ditto").send().await?.json().await?;
    let body_1=&json["flavor_text_entries"][0]["flavor_text"];
    let body_2=&json["name"];
    log::info!("{:?}", body_2);
    log::info!("{:?}",body_1);
    Ok(())
}
