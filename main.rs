use actix_web::{web, App, Error, HttpResponse, HttpServer, middleware};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::env;
use std::fs;
use std::io::Write;

async fn optimize_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let file_name = format!("{}.png", uuid::Uuid::new_v4().to_string());
        let mut out = web::block(|| std::fs::File::create(&file_name))
            .await
            .unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            out = web::block(move || out.write_all(&data).map(|_| out)).await?;
        }
    }

    Ok(HttpResponse::Ok().into())
}

fn error_handlers() -> middleware::ErrorHandlers {
    middleware::ErrorHandlers::new()
        .handler(404, |res| {
            HttpResponse::NotFound().body("The resource you tried to access was not found!")
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(error_handlers())
            .service(
                web::resource("/upload")
                    .route(web::post().to(optimize_image)),
            )
    })
    .bind(&server_address)?
    .run()
    .await
}