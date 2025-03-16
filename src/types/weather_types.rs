use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

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
