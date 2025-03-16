use std::error::Error;
use teloxide::Bot;
use teloxide::prelude::Requester;
use crate::controllers::weather_controller::WeatherForecast;

pub async fn send_bot_message_forecast(weather: &WeatherForecast) -> Result<teloxide::prelude::Message, Box<dyn Error>> {
    let channel_id = dotenv::var("CHANNEL_ID")?;
    let bot = Bot::from_env();

    let min = &weather.daily.temperature_2m_min[0];
    let max = &weather.daily.temperature_2m_max[0];
    let chance_of_rain = &weather.daily.precipitation_probability_max[0];

    let message =
        format!("The weather today will be a minimum temperature of {min}C and a maximum of {max}C and a {chance_of_rain}% chance of rain.");

    let response = bot
        .send_message(String::from(channel_id), message)
        .await?;

    Ok(response)
}

pub async fn send_bot_message(message: &str) -> Result<teloxide::prelude::Message, Box<dyn Error>> {
    let channel_id = dotenv::var("CHANNEL_ID")?;
    let bot = Bot::from_env();

    let response = bot
        .send_message(String::from(channel_id), message)
        .await?;

    Ok(response)
}