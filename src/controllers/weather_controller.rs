use crate::telegram::send_bot_message;
use crate::types::weather_types::{LocationWeatherRequest, WeatherForecastRequest};
use crate::weather::{
    format_rain_prediction, get_lat_long_for_location, get_rain_prediction, get_seven_day_forecast,
};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/weather/seven-day-weather-forecast")]
pub async fn seven_day_weather_forecast(
    req: web::Query<LocationWeatherRequest>,
) -> anyhow::Result<impl Responder, actix_web::Error> {
    let location = req.location.clone();

    let (lat, long, timezone) = get_lat_long_for_location(location).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Location lookup error: {}", e))
    })?;

    let response = get_seven_day_forecast(lat, long, &timezone)
        .await
        .map_err(|err| {
            actix_web::error::ErrorInternalServerError(format!("Location lookup error: {}", err))
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[get("/weather/weather-forecast")]
pub async fn weather_forecast(
    req: web::Query<WeatherForecastRequest>,
) -> crate::error::Result<impl Responder> {
    let lat = req.lat;
    let long = req.long;
    let timezone = &req.timezone;

    Ok(crate::weather::get_weather(lat, long, timezone).await)
}

#[get("/weather/rain-prediction")]
pub async fn location_rain_prediction(
    req: web::Query<LocationWeatherRequest>,
) -> crate::error::Result<impl Responder> {
    let (lat, long, timezone) = get_lat_long_for_location(req.location.clone()).await?;

    let response = get_rain_prediction(lat, long, &timezone).await?;

    let _ = send_bot_message(&format_rain_prediction(&response)).await;

    Ok(response)
}
