
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WeatherResponse {
    temperature: f32,
    feels_like: f32,
    humidity: f32,
}

async fn weather() -> impl Responder {
    let response = WeatherResponse {
        temperature: 22.5,
        feels_like: 20.5,
        humidity: 45.0,
    };
    HttpResponse::Ok().json(response)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/weather", web::get().to(weather))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
