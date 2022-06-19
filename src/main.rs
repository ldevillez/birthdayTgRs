use config;

use teloxide::{prelude2::*, utils::command::BotCommand};
use std::error::Error;
use serde;

use tokio::time::{self, Duration};

use chrono::prelude::{DateTime, Local, Datelike};

mod bth;
mod database;
mod user_config;
mod mytime;



type AsyncResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(serde::Deserialize)]
pub struct Conf {
    bot_token: String,
    user_id: i64, // For admin purpose
    db_name: String, //path to db
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
    #[command(description = "Get your user config")]
    FetchConfig,
    #[command(description = "Update your user config: hour minute")]
    UpdateConfig(i32, i32),
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
                    msg.push_str(&format!("- {}: {:0>2}/{:0>2} (id: {}) (reminder: {})\n", res.name, res.day, res.month, res.id, res.reminder ));
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
                    msg.push_str(&format!("- {}: {:0>2}/{:0>2} (id: {}) (reminder: {})\n", res.name, res.day, res.month, res.id, res.reminder ));
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
                    msg.push_str(&format!("- {}: {:0>2}/{:0>2} (id: {}) (reminder: {})\n", res.name, res.day, res.month, res.id, res.reminder ));
                }
                bot.send_message(message.chat.id, msg).await?
            }
        }
        Command::Add(name, day, month, reminder) => {
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
                let conf = database::get_config(message.chat.id as i32).await;
                match conf {
                    Ok(_) => {
                        //Has conf, do nothing
                    },
                    Err(_) => {
                        let myconf = user_config::UserConfig{
                            hour: 9,
                            minute: 0
                        };
                        let _res = database::create_config(message.chat.id as i32,&myconf).await;
                        msg.push_str("Creating user config ! \n");
                    }
                }

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

        Command::FetchConfig => {
            let mut msg: String = "".to_owned();
            let conf = database::get_config(message.chat.id as i32).await;
            match conf {
                Ok(myconf) => {
                    msg.push_str(&format!("Your reminder is scheduled for {:0>2}h{:0>2}", myconf.hour, myconf.minute));
                },
                Err(_) => {
                    msg.push_str("You have no user config");
                }
            }
            bot.send_message(message.chat.id,msg).await?
        }
        Command::UpdateConfig(hour, minute) => {
            let mut msg: String = "".to_owned();
            let conf = database::get_config(message.chat.id as i32).await;
            match conf {
                Ok(mut myconf) => {
                    myconf.hour = hour;
                    myconf.minute = minute;
                    if user_config::check_user_config(&myconf) {
                        let res = database::create_config(message.chat.id as i32,&myconf).await;
                        match res {
                            Ok(_) => {
                                msg.push_str("Your config is updated!");
                            },
                            Err(_) => {
                                msg.push_str("Error creating your config");
                            }
                        }
                    } else {
                        msg.push_str("Your config is not valid");
                    }
                },
                Err(_) => {
                    let myconf = user_config::UserConfig{
                        hour: hour,
                        minute: minute,
                    };
                    if user_config::check_user_config(&myconf) {
                        let res = database::create_config(message.chat.id as i32,&myconf).await;
                        match res {
                            Ok(_) => {
                                msg.push_str("Your config is created!");
                            },
                            Err(_) => {
                                msg.push_str("Error creating your config");
                            }
                        }
                    } else {
                        msg.push_str("Your config is not valid");
                    }
                }
            }
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

    println!("Loading conf");
    let conf: Conf = settings.try_into().unwrap();

    database::set_db(conf.db_name).await;

    println!("Initializing database");
    let status_db = database::initialize_database();
    status_db.await.expect("Could not initialize database");


    println!("Starting bot...");
    let bot = Bot::new(&conf.bot_token).auto_send();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));
        let bot_interval = Bot::new(&conf.bot_token).auto_send();
        let mut previous = mytime::get_mytime_from_time();
        interval.tick().await;

        loop {
            interval.tick().await;
            let current = mytime::get_mytime_from_time();
            let ids:Vec<i32> = database::get_all_config(&previous,&current).await.unwrap();
            for id in ids {
                let mut vec_bth = bth::get_reminded_birthday(id).await;
                if vec_bth.len() > 0 {
                    let mut msg: String = "Your comming soon birthdays:\n".to_owned();
                    bth::sort_bths_name(&mut vec_bth);
                    for bt in vec_bth {
                        msg.push_str(&format!("- {}: {:0>2}/{:0>2} (id: {}) (reminder: {})\n", bt.name, bt.day, bt.month, bt.id, bt.reminder ));

                    }
                    let status = bot_interval.send_message(id as i64, msg).await;
                }
            }
            previous = current;
            //TODO log status
        }

    });

    teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;

// TODO logging
}
