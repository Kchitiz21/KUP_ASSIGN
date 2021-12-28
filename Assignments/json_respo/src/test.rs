#[cfg(test)]
pub mod tests {
    use serde_json::Value::{Null, String};


    #[tokio::test]
    pub async fn body_success() {
        let body = String("ditto".to_string());
        let json: serde_json::Value = reqwest::Client::new().get("https://pokeapi.co/api/v2/pokemon-species/ditto").send().await.unwrap().json().await.unwrap();
        let body_2 = &json["name"];
        assert_eq!(*body_2, body)
    }
    #[tokio::test]
    pub async fn body_fail() {
        let body = Null;
        let json: serde_json::Value = reqwest::Client::new().get("https://pokeapi.co/api/v2/pokemon-species/ditto").send().await.unwrap().json().await.unwrap();
        let body_3 = &json["nae"];
        assert_eq!(*body_3, body)
    }
}
