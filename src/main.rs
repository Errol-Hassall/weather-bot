use actix_web::{App, HttpServer};
use health::health_check;
use teloxide::prelude::*;
use weather::weather_forcast;

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
        teloxide::repl(bot, |bot: Bot, msg: Message| async move {
            bot.send_dice(msg.chat.id).await?;
            Ok(())
        })
        .await;
    });

    HttpServer::new(|| App::new().service(health_check).service(weather_forcast))
        .bind(("0.0.0.0", 4000))?
        .run()
        .await
}
