use actix_web::{App, HttpServer};
use health::health_check;
use teloxide::{prelude::*, utils::command::BotCommands};
use weather::{seven_day_weather_forecast, weather_forecast};
use crate::weather::location_rain_prediction;

mod health;
mod weather;
mod telegram;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    let bot = Bot::from_env();

    // Spawns the telegram bot.
    // This is done here because its a long running blocking operation
    // You can't have two blocking processes in a single thread
    tokio::spawn(async move {
        Command::repl(bot, answer).await;
    });

    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(weather_forecast)
            .service(seven_day_weather_forecast)
            .service(location_rain_prediction)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "sends a weekly forecast.")]
    Forecast(String),
    #[command(description = "sends a forecast of the amount of rain for the day.")]
    Rain(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Forecast(location) => {
            let location = weather::get_lat_long_for_location(location).await;

            let forecast = weather::get_seven_day_forecast(location.0, location.1, &location.2).await.unwrap();

            bot.send_message(msg.chat.id, weather::format_weekly_forecast(&forecast))
                .await?
        }
        Command::Rain(location) => {
            let location = weather::get_lat_long_for_location(location).await;

            let forecast = weather::get_rain_prediction(location.0, location.1, &location.2).await.unwrap();

            bot.send_message(msg.chat.id, weather::format_rain_prediction(&forecast))
                .await?
        }
    };

    Ok(())
}
