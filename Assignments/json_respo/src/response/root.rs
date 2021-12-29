extern crate reqwest;
extern crate tokio;
use serde_json::Value::String;

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
pub async fn json(value:std::string::String) -> Result<std::string::String, reqwest::Error> {

    let json: serde_json::Value = reqwest::Client::new().get("https://pokeapi.co/api/v2/pokemon-species/ditto").send().await?.json().await?;

    let body=json[value].to_owned();
    let resp = match body{
        String(resp)=>resp,
        _ =>"Error".to_string()
    };
    log::info!("{:#?}",resp);
    Ok(resp)
}
