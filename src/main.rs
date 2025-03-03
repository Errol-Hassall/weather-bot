use actix_web::{App, HttpServer};
use health::health_check;
use teloxide::{prelude::*, utils::command::BotCommands};
use weather::{seven_day_weather_forcast, weather_forcast};

mod health;
mod weather;

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
            .service(weather_forcast)
            .service(seven_day_weather_forcast)
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
    #[command(description = "sends a weekly forcast.")]
    Forcast(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Forcast(location) => {
            let location = weather::get_lat_long_for_location(location).await;

            let forcast = weather::get_seven_day_forcast(location.0, location.1, &location.2).await;

            bot.send_message(msg.chat.id, weather::format_weekly_forcast(&forcast))
                .await?
        }
    };

    Ok(())
}
