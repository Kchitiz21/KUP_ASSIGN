pub mod test;
pub mod root {
    pub mod task_handler;
}
pub use actix_web::{web, App, HttpServer};
pub use root::task_handler::{create,delete,rename};
pub use serde_json;

/// Function
/// main: main function  creates a new HTTP Server
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Returns std::io::Result<()>
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    log::info!("Starting a HTTP Server");

    HttpServer::new(|| {
        App::new()
            .route("/delete", web::delete().to(delete))
            .route("/rename", web::put().to(rename))
            .route("/create", web::post().to(create))
    })
        .bind("127.0.0.1:880")?
        .run()
        .await
}