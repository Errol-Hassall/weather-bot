use actix_web::{
    body::BoxBody, get, http::header::ContentType, web, HttpRequest, HttpResponse, Responder,
    Result,
};

use serde::{Deserialize, Serialize};
use teloxide::{prelude::Requester, Bot};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Daily {
    time: Vec<String>,
    temperature_2m_max: Vec<f32>,
    temperature_2m_min: Vec<f32>,
}

impl Responder for Daily {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct WeatherForcast {
    daily: Daily,
}

// Responder
impl Responder for WeatherForcast {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/weather/weather-forcast")]
pub async fn weather_forcast() -> Result<impl Responder> {
    let url = String::from("https://api.open-meteo.com/v1/forecast?latitude=-37.5662&longitude=143.8496&daily=temperature_2m_max,temperature_2m_min&timezone=Australia%2FSydney&forecast_days=1");
    let response: WeatherForcast = reqwest::get(url)
        .await
        .unwrap()
        .json::<WeatherForcast>()
        .await
        .unwrap();

    let _ = send_bot_message(&response).await;

    Ok(web::Json(response))
}

async fn send_bot_message(weather: &WeatherForcast) -> teloxide::prelude::Message {
    let channel_id = dotenv::var("CHANNEL_ID").unwrap();
    let bot = Bot::from_env();

    let min = &weather.daily.temperature_2m_min[0];
    let max = &weather.daily.temperature_2m_max[0];

    let message = format!("Minimum temperature of {min} and a maximum of {max}");

    let response = bot
        .send_message(String::from(channel_id), message)
        .await
        .unwrap();

    response
}
