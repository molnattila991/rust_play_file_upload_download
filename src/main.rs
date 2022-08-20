use actix_multipart::Multipart;
use actix_web::{post, App, Error, HttpResponse, HttpServer};

pub mod files {
    use actix_multipart::Multipart;
    use futures::{StreamExt, TryStreamExt};
    use uuid::Uuid;

    pub async fn save_file(mut payload: Multipart) -> Option<bool> {
        while let Ok(Some(mut field)) = payload.try_next().await {
            let id = Uuid::new_v4();
            let extension = field
                .content_disposition()
                .get_filename()
                .unwrap()
                .split(".")
                .last()
                .unwrap();
            let path = format!("images/{}.{}", id, extension);

            while let Some(chunk) = field.next().await {
                let _ = tokio::fs::write(&path, &chunk.unwrap()).await;
            }
        }

        Some(true)
    }
}

#[post("/")]
async fn index(payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    let upload_status = files::save_file(payload).await;

    match upload_status {
        Some(true) => Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("update_succeeded")),
        _ => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("update_failed")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
