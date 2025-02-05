use actix_web::{get, HttpResponse, Responder};
use teloxide::{prelude::Requester, requests::JsonRequest, Bot};

#[get("/weather/weather-forcast")]
pub async fn weather_forcast() -> impl Responder {
    let response = reqwest::get("https://api.open-meteo.com/v1/forecast?latitude=-37.5662&longitude=143.8496&daily=temperature_2m_max,temperature_2m_min&timezone=Australia%2FSydney&forecast_days=1").await.unwrap().text().await.unwrap();

    let _ = send_bot_message(response.clone()).await;

    HttpResponse::Ok().body(response)
}

async fn send_bot_message(_weather: String) -> JsonRequest<teloxide::payloads::SendMessage> {
    let channel_id = dotenv::var("CHANNEL_ID").unwrap();
    let bot = Bot::from_env();

    bot.send_message(String::from(channel_id), "Test")
}
