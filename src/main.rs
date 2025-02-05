use std::{thread, time::Duration};

use actix_web::{App, HttpServer};
use health::health_check;
use weather::weather_forcast;

mod background_job;
mod health;
mod weather;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    thread::spawn(|| {
        while true {
            background_job::background_job();
        }
    });

    HttpServer::new(|| App::new().service(health_check).service(weather_forcast))
        .bind(("127.0.0.1", 4000))?
        .run()
        .await
}
