#[cfg(test)]
pub mod tests {
    #[tokio::test]
    pub async fn _reqwest() {
        let client = reqwest::Client::new();
        let res = client
            .get(&format!("https://www.googleapis.com/auth/calendar"))
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), reqwest::StatusCode::OK);
    }
}
#[tokio::test]
pub async fn _reqwest() {
    let client = reqwest::Client::new();
    let res = client
        .get(&format!("https://www.googleapis.com/auth/"))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::NOT_FOUND);
}
