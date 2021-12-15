extern crate reqwest;
use log::*;
extern crate tokio;
///_testing: Testing function is used to get the URL
///
/// Arguments
///
/// No Arguments
///
/// Return
///
/// Return Result<(), reqwest::Error

#[tokio::main]
pub async fn _testing() -> Result<(), reqwest::Error> {
    let res = reqwest::Client::new()
        .get("https://www.googleapis.com/auth/calendar")
        .send()
        .await?;

    info!("Status: {}", res.status());

    let text = res.text().await?;

   info!("Body:{}", text);

    Ok(())
}