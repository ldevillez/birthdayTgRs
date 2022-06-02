use config;

use teloxide::{prelude2::*, utils::command::BotCommand};
use std::error::Error;
use serde;

use tokio::sync::Mutex;

use rusqlite::{Connection, Result, params};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONNECTION: Mutex<Connection> = Mutex::new(Connection::open("./birthday.db3").unwrap());
}

type AsyncResult<T> = Result<T, Box<dyn Error + Send + Sync>>;


#[derive(serde::Deserialize)]
pub struct Conf {
    bot_token: String,
    user_id: i64,
    db_name: String,
}

#[derive(Debug)]
struct Birthday {
    name: String,
    day: i32,
    month: i32,
    reminder: i32,
    id: i32,
}


#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "List the following birthday")]
    Fetch,
}

async fn get_all_persons() -> AsyncResult<Vec<Birthday>> {
    let mut birthdays: Vec<Birthday> = Vec::new();
    let c = CONNECTION.lock().await;
    println!("Test ?");
    let mut statement = c.prepare("SELECT name, day, month, reminder FROM person")?;
    //let mut statement = c.prepare(&format!("SELECT name, day, month, reminder FROM person WHERE user_id={}", 1))?;
    println!("Test 2 ?");

    //while let sqlite::State::Row = statement.next().unwrap() {

    //}
    Ok(birthdays)
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

            match get_all_persons().await{
                Err(e) => bot.send_message(message.chat.id, "Uh, something went wrong \n").await?,
                Ok(birthdays) => bot.send_message(message.chat.id, "Good").await?
            };
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


            bot.send_message(message.chat.id, format!("Bla Bla ")).await?
        }
    };

    Ok(())
}

//async fn connect_database() -> AsyncResult<()> {
    //CONNECTION = Mutex::new(Connection::open("./birthday.db3").unwrap());

    //Ok(())
//}

async fn initialize_database() -> AsyncResult<()> {

  CONNECTION.lock().await.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id       INTEGER PRIMARY KEY,
            user_id  INTEGER,
            name     TEXT NOT NULL,
            day      INTEGER,
            month    INTEGER,
            reminder INTEGER
        )",
        [], // empty list of parameters.
    )?;
  Ok(())
}

async fn get_all_birthdays(user_id: i32) -> AsyncResult<Vec<Birthday>> {
    let mut birthdays: Vec<Birthday> = Vec::new();
    let c = CONNECTION.lock().await;
    let mut statement = c.prepare(&format!("SELECT name, day, month, reminder, id FROM person WHERE user_id ={}", user_id))?;
    let birthdays_map = statement.query_map([], |row| {
        Ok(Birthday {
            name: row.get(0)?,
            day: row.get(1)?,
            month: row.get(2)?,
            reminder: row.get(3)?,
            id: row.get(4)?,
        })
    })?;

    for birthday in birthdays_map {
        birthdays.push(birthday.unwrap())
    }
    Ok(birthdays)
}

async fn create_birthday(user_id: i32, birthday: &Birthday) -> AsyncResult<()> {
    CONNECTION.lock().await.execute("INSERT INTO person (user_id, name, day, month, reminder) VALUES (?1, ?2, ?3, ?4, ?5)", params![user_id, birthday.name, birthday.day, birthday.month, birthday.reminder],)?;

    Ok(())
}

async fn delete_birthday(user_id: i32, id: i32) -> AsyncResult<()>{
    CONNECTION.lock().await.execute("DELETE FROM person WHERE id=?1 AND user_id=?2", params![id,user_id],)?;

    Ok(())
}

async fn edit_birthday(user_id: i32, birthday: &Birthday) -> AsyncResult<()>{
    CONNECTION.lock().await.execute("UPDATE person SET name = ?1, day = ?2, month = ?3, reminder=?4 WHERE id = ?5 AND user_id = ?6", params![birthday.name, birthday.day, birthday.month, birthday.reminder, birthday.id,user_id],)?;

    Ok(())
}

#[tokio::main]
async fn main() {

    let path = "birthday_tg_rs";
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(path)).unwrap();

    println!("Initializing database");
    let status_db = initialize_database();
    status_db.await.expect("Could not initialize database");


    println!("Loading conf");
    let conf: Conf = settings.try_into().unwrap();
    //println!("Token is {}", conf.bot_token);
    //println!("User ID is {}",conf.user_id);

    let mut bth = Birthday {
        name: "test".to_owned(),
        day: 0,
        month: 1,
        reminder: 2,
        id: 0
    };
    let status_db = create_birthday(10, &bth);
    status_db.await.expect("Could not add birthday");
    bth.id = 3;
    bth.reminder = 4;
    let status_db = edit_birthday(10, &bth);
    status_db.await.expect("Could edit birthday");
    let status_db = delete_birthday(10, 2);
    status_db.await.expect("Could not delete birthday");
    let result = get_all_birthdays(10).await.unwrap();
    println!("Result: {:?}", result);

    println!("Starting bot...");
    let bot = Bot::new(conf.bot_token).auto_send();
    //teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}

// TODO logging
