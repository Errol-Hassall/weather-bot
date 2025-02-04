use actix_web::{get, HttpResponse, Responder};

#[get("/weather/weather-forcast")]
pub async fn weather_forcast() -> impl Responder {
    // let lat = "-37.562160";
    // let long = "143.850250";

    let response = reqwest::get("https://api.open-meteo.com/v1/forecast?latitude=-37.5662&longitude=143.8496&daily=temperature_2m_max,temperature_2m_min&timezone=Australia%2FSydney&forecast_days=1").await.unwrap().text().await.unwrap();
    HttpResponse::Ok().body(response)
}
