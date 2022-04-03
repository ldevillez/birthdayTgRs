use config;

use teloxide::{prelude2::*, utils::command::BotCommand};
use std::error::Error;
use serde;

use rusqlite::{Connection, Result};

#[derive(serde::Deserialize)]
pub struct Conf {
    bot_token: String,
    user_id: i64,
    db_name: String,
}


#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "List the following birthday")]
    Fetch,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Only regular user
    if message.chat.id < 0 {
        return Ok(())
    }
    match command {
        Command::Help => bot.send_message(message.chat.id, Command::descriptions()).await?,
        Command::Fetch => {

            bot.send_message(message.chat.id, format!("Bla Bla")).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
  let path = "birthday_tg_rs";
  let mut settings = config::Config::default();
  settings
    .merge(config::File::with_name(path)).unwrap();

  let conf: Conf = settings.try_into().unwrap();
  println!("Token is {}", conf.bot_token);
  println!("User ID is {}",conf.user_id);

  let conn = Connection::open(conf.db_name).unwrap();
  let status = conn.execute(
        "CREATE TABLE person (
            id       INTEGER PRIMARY KEY,
            user_id  INTEGER,
            name     TEXT NOT NULL,
            day      INTEGER,
            month    INTEGER,
            reminder INTEGER,
        )",
        [], // empty list of parameters.
    );

  // TODO check status to continue or not


  pretty_env_logger::init();
  log::info!("Starting dices_bot...");
  let bot = Bot::new(conf.bot_token).auto_send();
  teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}
