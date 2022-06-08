use config;

use teloxide::{prelude2::*, utils::command::BotCommand};
use std::error::Error;
use serde;

use tokio::runtime::Runtime;
use tokio::time::{self, Duration};

mod bth;

mod database;




type AsyncResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

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
) -> AsyncResult<()> {
    // Only regular user
    if message.chat.id < 0 {
        return Ok(())
    }
    match command {
        Command::Help => bot.send_message(message.chat.id, Command::descriptions()).await?,
        Command::Fetch => {

            //let c = CONNECTION.lock().await;
            //let mut statement = c.prepare(&format!("SELECT name, day, month, reminder FROM person WHERE user_id={}", 1))?;

            //let mut stmt = conn.prepare(
                //"SELECT name, day, month, reminder FROM person WHERE user_id = ?1"
            //).unwrap();

            //let bd_iter = stmt.query_map(&[&message.chat.id], |row| {
                //Ok(Birthday {
                    //name: row.get(0)?,
                    //day: row.get(1)?,
                    //month: row.get(2)?,
                    //reminder: row.get(3)?,
                //})
            //});

            //for person in bd_iter {
                //for test in person {
                    ////println!("Found person {:?}", test);
                //}
            //}


            println!("Chat: {:?}", message.chat.id);
            bot.send_message(message.chat.id, format!("Bla Bla ")).await?
        }
    };

    Ok(())
}

//async fn connect_database() -> AsyncResult<()> {
    //CONNECTION = Mutex::new(Connection::open("./birthday.db3").unwrap());

    //Ok(())
//}

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
        name: "test".to_owned(),
        day: 0,
        month: 1,
        reminder: 2,
        id: 3
    };
    let status_db = database::create_birthday(10, &bth);
    status_db.await.expect("Could not add birthday");
    bth.reminder = 4;
    let status_db = database::edit_birthday(10, &bth);
    status_db.await.expect("Could edit birthday");
    let status_db = database::delete_birthday(10, 2);
    status_db.await.expect("Could not delete birthday");
    let result = database::get_all_birthdays(10).await.unwrap();
    println!("Result: {:?}", result);

    println!("Starting bot...");
    let bot = Bot::new(&conf.bot_token).auto_send();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        let bot_interval = Bot::new(&conf.bot_token).auto_send();

        loop {
            interval.tick().await;
            let status = bot_interval.send_message(357669106, "Hey").await;
            //TODO log status
        }

    });

    //bot.send_message(357669106, "Hey").await;
    teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}

// TODO logging
