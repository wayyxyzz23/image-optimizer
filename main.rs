use actix_web::{web, App, Error, HttpResponse, HttpServer, http::StatusCode, middleware, dev::HttpResponseBuilder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::env;
use std::io::Write;

async fn optimize_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // Iterating through the fields of the multipart form
    while let Ok(Some(mut field)) = payload.try_next().await {
        let file_name = format!("{}.png", uuid::Uuid::new_v4().to_string());

        // Safely attempting to create a file and preparing for write operations
        let create_file_result = web::block(|| std::fs::File::create(&file_name)).await;

        // Check if file creation was successful
        let mut out = match create_file_result {
            Ok(file) => file,
            Err(e) => {
                // Logging or handling the error as needed
                eprintln!("Error creating file: {:?}", e);
                return Err(Error::from(HttpResponse::InternalServerError().body("Unable to create file")));
            }
        };

        // Writing data to file
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| Error::from(HttpResponse::InternalServerError().body("Error reading file chunk")))?;
            out = web::block(move || out.write_all(&data).map(|_| out))
                .await
                .map_err(|_| Error::from(HttpResponse::InternalServerError().body("Error saving file chunk")))?;
        }
    }

    // Responding with success once all fields have been processed
    Ok(HttpResponse::Ok().into())
}

fn error_handlers() -> middleware::ErrorHandlers {
    middleware::ErrorHandlers::new()
        .handler(404, |res| {
            HttpResponse::NotFound().body("The resource you tried to access was not found!")
        })
        .handler(500, |_| {
            HttpResponse::InternalServerError().body("An internal server error occurred!")
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Fetching server address from environment or defaulting if not set
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // Setting up the server
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