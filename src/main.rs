use config;

use teloxide::{prelude2::*, utils::command::BotCommand};
use std::error::Error;
use serde;

use tokio::time::{self, Duration};

use chrono::prelude::{DateTime, Local, Datelike};

mod bth;

mod database;




type AsyncResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(serde::Deserialize)]
pub struct Conf {
    bot_token: String,
    user_id: i64,
    db_name: String,
}



#[derive(BotCommand, Clone, PartialEq)]
#[command(rename = "lowercase", description = "These commands are supported:", parse_with="split")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "List the following birthday with the following order")]
    Fetch,
    #[command(description = "List the following birthday with alphabetical order")]
    FetchAlpha,
    #[command(description = "List the following birthday with date order")]
    FetchOrder,
    #[command(description = "Add a birthday: name, month, day, reminder offset")]
    Add(String,i32,i32,i32),
    #[command(description = "Delete a birthday: id")]
    Delete(i32),
}


async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> AsyncResult<()> {
    // Only regular user
    if message.chat.id < 0 {
        return Ok(())
    }
    match command {
        Command::Help => bot.send_message(message.chat.id, Command::descriptions()).await?,
        Command::Fetch => {
            let mut result = database::get_all_birthdays(message.chat.id as i32).await.unwrap();
            if result.len() == 0 {
                bot.send_message(message.chat.id, "You have no reminder start by creating one").await?
            } else {
                let local: DateTime<Local> = Local::now();
                bth::sort_bths_after_date(&mut result, local.month() as i32, local.day() as i32);
                let mut msg: String = "You have the following birthdays:\n".to_owned();
                for res in result {
                    msg.push_str(&format!("- {}: {}/{} (id: {})\n", res.name, res.month, res.day, res.id ));
                }
                bot.send_message(message.chat.id, msg).await?
            }
        }
        Command::FetchAlpha => {
            let mut result = database::get_all_birthdays(message.chat.id as i32).await.unwrap();
            if result.len() == 0 {
                bot.send_message(message.chat.id, "You have no reminder start by creating one").await?
            } else {
                bth::sort_bths_name(&mut result);
                let mut msg: String = "You have the following birthdays:\n".to_owned();
                for res in result {
                    msg.push_str(&format!("- {}: {}/{} (id: {})\n", res.name, res.month, res.day, res.id ));
                }
                bot.send_message(message.chat.id, msg).await?
            }
        }
        Command::FetchOrder => {
            let mut result = database::get_all_birthdays(message.chat.id as i32).await.unwrap();
            if result.len() == 0 {
                bot.send_message(message.chat.id, "You have no reminder start by creating one").await?
            } else {
                bth::sort_bths_date(&mut result);
                let mut msg: String = "You have the following birthdays:\n".to_owned();
                for res in result {
                    msg.push_str(&format!("- {}: {}/{} (id: {})\n", res.name, res.month, res.day, res.id ));
                }
                bot.send_message(message.chat.id, msg).await?
            }
        }
        Command::Add(name, month, day, reminder) => {
            let bth = bth::Birthday {
                name: name,
                day: day,
                month: month,
                reminder: reminder,
                id: 0
            };
            if !bth::check_bth(&bth){
                bot.send_message(message.chat.id,"Could not parse to a valid birthday").await?
            }  else {
                let status_db = database::create_birthday(message.chat.id as i32, &bth);

                let mut msg: String = "".to_owned();
                match status_db.await {
                    Ok(_result) => msg.push_str("Birthady added !"),
                    Err(_e) => msg.push_str("Could not add Birthday to database")
                };
                bot.send_message(message.chat.id,msg).await?
            }
        }


        Command::Delete(id) => {
            let status_db = database::delete_birthday(message.chat.id as i32, id);

            let mut msg: String = "".to_owned();
            match status_db.await {
                Ok(_result) => msg.push_str(&format!("Birthady (id: {}) deleted !", id)),
                Err(_e) => msg.push_str(&format!("Could not delete Birthday (id: {})",id))
            };
            bot.send_message(message.chat.id,msg).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {

    let path = "birthday_tg_rs";
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(path)).unwrap();

    println!("Initializing database");
    let status_db = database::initialize_database();
    status_db.await.expect("Could not initialize database");


    println!("Loading conf");
    let conf: Conf = settings.try_into().unwrap();
    //println!("Token is {}", conf.bot_token);
    //println!("User ID is {}",conf.user_id);

    let mut bth = bth::Birthday {
        name: "Muncho".to_owned(),
        day: 21,
        month: 8,
        reminder: 6,
        id: 3
    };
    let status_db = database::create_birthday(357669106, &bth);
    status_db.await.expect("Could not add birthday");
    //bth.reminder = 4;
    //let status_db = database::edit_birthday(10, &bth);
    //status_db.await.expect("Could edit birthday");
    //let status_db = database::delete_birthday(10, 2);
    //status_db.await.expect("Could not delete birthday");
    //let result = database::get_all_birthdays(10).await.unwrap();
    //println!("Result: {:?}", result);

    println!("Starting bot...");
    let bot = Bot::new(&conf.bot_token).auto_send();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        let bot_interval = Bot::new(&conf.bot_token).auto_send();

        loop {
            interval.tick().await;
            //let status = bot_interval.send_message(357669106, "Hey").await;
            //TODO log status
        }

    });

    //bot.send_message(357669106, "Hey").await;
    teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}

// TODO logging
