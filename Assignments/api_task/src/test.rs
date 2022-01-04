#[cfg(test)]
pub mod tests {
   pub use crate::root::task_handler::{create,delete,rename};
    pub use serde_json;
    pub use serde_json::json;

    #[actix_web::test]
    pub async fn create_sucess() {
        env_logger::init();
        assert_eq!(
            create("src/img.txt".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": true})).unwrap()
        );
        log::info!(" Created Successfully");
    }
    #[actix_web::test]
    pub async fn create_fail() {
        assert_eq!(
            create("sac/img.txt".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": false})).unwrap()
        );
        log::info!("Not found");
    }


    #[actix_web::test]
    pub async fn rename_sucess() {
        assert_eq!(
            rename("src/img.txt".to_string(), "src/html.txt".to_string())
                .await
                .unwrap(),
            serde_json::to_string_pretty(&json!({"status": true})).unwrap()
        );
        log::info!("Renamed Successfully");
    }
    /// Negative test case [status: false]
    #[actix_web::test]
    pub async fn rename_fail() {
        assert_eq!(
            rename("sac/img.txt".to_string(), "sac/html.txt".to_string())
                .await
                .unwrap(),
            serde_json::to_string_pretty(&json!({"status": false})).unwrap()
        );
        log::info!("Not found");
    }

    /// Testing deletion of file at the specific location
    /// Positive test case [status: true]
    #[actix_web::test]
    pub async fn delete_sucess() {
        assert_eq!(
            delete("src/html.txt".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status":true})).unwrap()
        );
        log::info!("Deleted Successfully");
    }

    #[actix_web::test]
    pub async fn delete_fail() {
        assert_eq!(
            delete("sac/img.txt".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": false})).unwrap()
        );
        log::info!("Not found");
    }
}
