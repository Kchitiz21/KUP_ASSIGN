#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    time: u64,
    err: String
}

#[derive(Deserialize)]
struct Download {
    name: String,
}

async fn upload() -> impl Responder {
    let u = &File {
        name: "dummy data".to_string(),
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        err: "".to_string()
    };
    HttpResponse::Ok().json(u)
}

async fn download(info: web::Path<Download>) -> HttpResponse {
    let name = String::from(info.name.as_str());
    let body = once(ok::<_, actix_web::Error>(Bytes::from(name)));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}