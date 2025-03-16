use actix_web::body::BoxBody;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use crate::telegram::send_bot_message;
use crate::weather::{format_rain_prediction, get_lat_long_for_location, get_rain_prediction, get_seven_day_forecast};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Daily {
    pub(crate) time: Vec<String>,
    pub(crate) temperature_2m_max: Vec<f64>,
    pub(crate) temperature_2m_min: Vec<f64>,
    pub(crate) precipitation_probability_max: Vec<i32>,
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
pub struct WeatherForecast {
    pub(crate) daily: Daily,
}

// Responder
impl Responder for WeatherForecast {
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
pub struct LocationWeatherRequest {
    pub(crate) location: String,
}

#[derive(Debug, Deserialize)]
pub struct WeatherForecastRequest {
    pub(crate) lat: f64,
    pub(crate) long: f64,
    pub(crate) timezone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocationResult {
    id: u32,
    name: String,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) timezone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocationResponse {
    pub(crate) results: Vec<LocationResult>,
}

#[get("/weather/seven-day-weather-forecast")]
pub async fn seven_day_weather_forecast(
    req: web::Query<LocationWeatherRequest>,
) -> crate::error::Result<impl Responder> {
    let (lat, long, timezone) = get_lat_long_for_location(req.location.clone()).await;

    let response = get_seven_day_forecast(lat, long, &timezone).await?;

    Ok(web::Json(response))
}

#[get("/weather/weather-forecast")]
pub async fn weather_forecast(req: web::Query<WeatherForecastRequest>) -> crate::error::Result<impl Responder> {
    let lat = req.lat;
    let long = req.long;
    let timezone = &req.timezone;

    Ok(crate::weather::get_weather(lat, long, timezone).await)
}

#[get("/weather/rain-prediction")]
pub async fn location_rain_prediction(
    req: web::Query<LocationWeatherRequest>,
) -> crate::error::Result<impl Responder> {
    let (lat, long, timezone) = get_lat_long_for_location(req.location.clone()).await;

    let response = get_rain_prediction(lat, long, &timezone).await?;

    let _ = send_bot_message(&format_rain_prediction(&response)).await;

    Ok(response)
}