use actix_multipart::Multipart;
use actix_web::{post, App, Error, HttpResponse, HttpServer};

pub mod files {
    use std::io::Write;

    use actix_multipart::Multipart;
    use actix_web::web;
    use futures::{StreamExt, TryStreamExt};
    use imagesize::size;

    pub async fn save_file(mut payload: Multipart, file_path: String) -> Option<bool> {
        // iterate over multipart stream
        while let Ok(Some(mut field)) = payload.try_next().await {
            // File::create is blocking operation, use threadpool
            let mut f = std::fs::File::create(file_path.clone()).unwrap();
                       
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // println!("{:?}", data.to_ascii_lowercase());
                // filesystem operations are blocking, we have to use threadpool
                let res = f.write_all(&data).unwrap();
            }
        }
        let img = image::open(file_path.clone()).unwrap();
        let alma = img.resize(100, 100, image::imageops::FilterType::Nearest);
        let muci = alma.save("path.png").unwrap();

        // image::load_from_memory(buffer)

        let valami = size(file_path).unwrap();
        println!("{} {}", valami.width, valami.height);

        Some(true)
    }
}

#[post("/")]
async fn index(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    let upload_status = files::save_file(payload, "./filename.png".to_string()).await;

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
