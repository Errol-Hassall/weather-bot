use actix_web::{
    body::BoxBody,
    get,
    http::header::ContentType,
    web::{self},
    HttpRequest, HttpResponse, Responder, Result,
};

use serde::{Deserialize, Serialize};
use teloxide::{prelude::Requester, Bot};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Daily {
    time: Vec<String>,
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
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
pub struct WeatherForcast {
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

#[derive(Debug, Deserialize, Clone)]
struct SevenDayWeatherForcastRequest {
    location: String,
}

#[derive(Debug, Deserialize)]
struct WeatherForcastRequest {
    lat: f64,
    long: f64,
    timezone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocationResult {
    id: u32,
    name: String,
    latitude: f64,
    longitude: f64,
    timezone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocationResponse {
    results: Vec<LocationResult>,
}

#[get("/weather/seven-day-weather-forcast")]
pub async fn seven_day_weather_forcast(
    req: web::Query<SevenDayWeatherForcastRequest>,
) -> Result<impl Responder> {
    let (lat, long, timezone) = get_lat_long_for_location(req.location.clone()).await;

    let response = get_seven_day_forcast(lat, long, &timezone).await;

    Ok(web::Json(response))
}

pub async fn get_lat_long_for_location(location: String) -> (f64, f64, String) {
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={location}&count=10&language=en&format=json");
    let response = reqwest::get(url)
        .await
        .unwrap()
        .json::<LocationResponse>()
        .await
        .unwrap();

    if response.results.len() < 1 {
        panic!("error");
    }

    (
        response.results[0].latitude,
        response.results[0].longitude,
        response.results[0].timezone.clone(),
    )
}

pub async fn get_seven_day_forcast(lat: f64, long: f64, timezone: &String) -> WeatherForcast {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={long}&daily=temperature_2m_max,temperature_2m_min,precipitation_probability_max&timezone={timezone}&forecast_days=7");
    let response: WeatherForcast = reqwest::get(url)
        .await
        .unwrap()
        .json::<WeatherForcast>()
        .await
        .unwrap();

    response
}

pub fn format_weekly_forcast(forcast: &WeatherForcast) -> String {
    let time = forcast.daily.time.clone();
    let min = forcast.daily.temperature_2m_min.clone();
    let max = forcast.daily.temperature_2m_max.clone();
    let rain = forcast.daily.precipitation_probability_max.clone();

    let mut message = "".to_string();

    for i in 1..7 {
        let t = &time[i];
        let mn = &min[i];
        let mx = &max[i];
        let r = &rain[i];
        message.push_str(&format!("date: {t} has a max temperature of {mx} and a low of {mn}. There is a {r}% chance of rain. \n"))
    }

    message
}

#[get("/weather/weather-forcast")]
pub async fn weather_forcast(req: web::Query<WeatherForcastRequest>) -> Result<impl Responder> {
    let lat = req.lat;
    let long = req.long;
    let timezone = &req.timezone;

    get_weather(lat, long, timezone).await
}

async fn get_weather(lat: f64, long: f64, timezone: &String) -> Result<impl Responder> {
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
        format!("The weather today will be a minimum temperature of {min}C and a maximum of {max}C and a {chance_of_rain}% chance of rain.");

    let response = bot
        .send_message(String::from(channel_id), message)
        .await
        .unwrap();

    response
}
