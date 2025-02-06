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
    precipitation_probability_max: Vec<i32>,
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

#[derive(Debug, Deserialize)]
struct WeatherForcastRequest {
    lat: f32,
    long: f32,
    timezone: String,
}

#[get("/weather/weather-forcast")]
pub async fn weather_forcast(req: web::Query<WeatherForcastRequest>) -> Result<impl Responder> {
    let lat = req.lat;
    let long = req.long;
    let timezone = &req.timezone;

    get_weather(lat, long, timezone).await
}

async fn get_weather(lat: f32, long: f32, timezone: &String) -> Result<impl Responder> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={long}&daily=temperature_2m_max,temperature_2m_min,precipitation_probability_max&timezone={timezone}&forecast_days=1");
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
    let chance_of_rain = &weather.daily.precipitation_probability_max[0];

    let message =
        format!("The weather today will be a minimum temperature of {min}C and a maximum of {max}C and a {chance_of_rain}% change of rain.");

    let response = bot
        .send_message(String::from(channel_id), message)
        .await
        .unwrap();

    response
}
