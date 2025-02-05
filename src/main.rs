use actix_web::{App, HttpServer};
use health::health_check;
use weather::weather_forcast;

mod health;
mod weather;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    HttpServer::new(|| App::new().service(health_check).service(weather_forcast))
        .bind(("0.0.0.0", 4000))?
        .run()
        .await
}
