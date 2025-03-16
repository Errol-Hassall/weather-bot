use actix_web::{
    body::BoxBody,
    http::header::ContentType,
    web::{self},
    HttpRequest, HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::telegram::{send_bot_message_forecast};
use crate::controllers::weather_controller::{LocationResponse, WeatherForecast};

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

pub async fn get_seven_day_forecast(lat: f64, long: f64, timezone: &String) -> Result<WeatherForecast> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={long}&daily=temperature_2m_max,temperature_2m_min,precipitation_probability_max&timezone={timezone}&forecast_days=7");
    let response: WeatherForecast = reqwest::get(url)
        .await.unwrap()
        .json::<WeatherForecast>()
        .await.unwrap();

    Ok(response)
}

pub fn format_weekly_forecast(forecast: &WeatherForecast) -> String {
    let time = forecast.daily.time.clone();
    let min = forecast.daily.temperature_2m_min.clone();
    let max = forecast.daily.temperature_2m_max.clone();
    let rain = forecast.daily.precipitation_probability_max.clone();

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


pub async fn get_weather(lat: f64, long: f64, timezone: &String) -> Result<impl Responder> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={long}&daily=temperature_2m_max,temperature_2m_min,precipitation_probability_max&timezone={timezone}&forecast_days=1");
    let response: WeatherForecast = reqwest::get(url)
        .await.unwrap()
        .json::<WeatherForecast>()
        .await.unwrap();

    let _ = send_bot_message_forecast(&response).await;

    Ok(web::Json(response))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hourly {
    time: Vec<String>,
    pub(crate) precipitation: Vec<f64>,
    pub(crate) precipitation_probability: Vec<f64>,
    pub(crate) rain: Vec<f64>,
    pub(crate) showers: Vec<f64>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RainPrediction {
    pub(crate) hourly: Hourly,
}

impl Responder for RainPrediction {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}


pub async fn get_rain_prediction(lat: f64, long: f64, timezone: &String) -> Result<RainPrediction> {
    let url = format!("https://api.open-meteo.com/v1/forecast?timezone={timezone}&latitude={lat}&longitude={long}&hourly=precipitation,precipitation_probability,rain,showers&forecast_days=1");

    let response: RainPrediction = reqwest::get(url)
        .await?
        .json::<RainPrediction>()
        .await?;

    Ok(response)
}

pub fn format_rain_prediction(prediction: &RainPrediction) -> String {
    let precipitation_total = prediction.hourly.precipitation.iter().sum::<f64>().round().to_string();

   format!("{precipitation_total}mm of rain is expected today")
}

