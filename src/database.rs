
use std::error::Error;
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use rusqlite::{Connection, Result, params};
use std::cell::RefCell;

type AsyncResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

use crate::bth;
use crate::user_config;
use crate::mytime;


lazy_static! {
    static ref CONNECTION: Mutex<RefCell<Connection>> = Mutex::new(RefCell::new(Connection::open("./birthday.db3").unwrap()));
}

pub async fn initialize_database() -> AsyncResult<()> {

  CONNECTION.lock().await.borrow().execute(
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
  CONNECTION.lock().await.borrow().execute(
        "CREATE TABLE IF NOT EXISTS config (
            id       INTEGER PRIMARY KEY,
            user_id  INTEGER,
            hour     INTERER,
            minute      INTEGER
        )",
        [], // empty list of parameters.
    )?;
  Ok(())
}

pub async fn get_all_birthdays(user_id: i32) -> AsyncResult<Vec<bth::Birthday>> {
    let mut birthdays: Vec<bth::Birthday> = Vec::new();
    for birthday in CONNECTION.lock().await.borrow().prepare(&format!("SELECT name, day, month, reminder, id FROM person WHERE user_id ={}", user_id))?.query_map([], |row| {
        Ok(bth::Birthday {
            name: row.get(0)?,
            day: row.get(1)?,
            month: row.get(2)?,
            reminder: row.get(3)?,
            id: row.get(4)?,
        })
    })? {
        birthdays.push(birthday.unwrap())
    }
    Ok(birthdays)
}

pub async fn create_birthday(user_id: i32, birthday: &bth::Birthday) -> AsyncResult<()> {
    CONNECTION.lock().await.borrow().execute("INSERT INTO person (user_id, name, day, month, reminder) VALUES (?1, ?2, ?3, ?4, ?5)", params![user_id, birthday.name, birthday.day, birthday.month, birthday.reminder],)?;

    Ok(())
}

pub async fn delete_birthday(user_id: i32, id: i32) -> AsyncResult<()>{
    CONNECTION.lock().await.borrow().execute("DELETE FROM person WHERE id=?1 AND user_id=?2", params![id,user_id],)?;

    Ok(())
}

pub async fn edit_birthday(user_id: i32, birthday: &bth::Birthday) -> AsyncResult<()>{
    CONNECTION.lock().await.borrow().execute("UPDATE person SET name = ?1, day = ?2, month = ?3, reminder=?4 WHERE id = ?5 AND user_id = ?6", params![birthday.name, birthday.day, birthday.month, birthday.reminder, birthday.id,user_id],)?;

    Ok(())
}

pub async fn get_config(user_id: i32) -> AsyncResult<user_config::UserConfig>{
    Ok(CONNECTION.lock().await.borrow().prepare(&format!("SELECT hour, minute FROM config WHERE user_id ={}", user_id))?.query_row([],|row| {
        //println!("Loadead, {:?}",row.get(0));
        Ok(user_config::UserConfig {
            hour: row.get(0)?,
            minute: row.get(1)?,
        })
    })?)
}

pub async fn edit_config(user_id: i32, config: &user_config::UserConfig) -> AsyncResult<()>{
    CONNECTION.lock().await.borrow().execute("UPDATE config SET hour = ?1, minute = ?2 WHERE user_id = ?3", params![config.hour, config.minute, user_id],)?;

    Ok(())
}

pub async fn create_config(user_id: i32, config: &user_config::UserConfig) -> AsyncResult<()>{
    CONNECTION.lock().await.borrow().execute("INSERT INTO config (hour, minute, user_id) VALUES (?1, ?2, ?3)", params![config.hour, config.minute, user_id],)?;

    Ok(())
}

pub async fn get_all_config(prev: &mytime::time, curr: &mytime::time) -> AsyncResult<Vec<i32>> {
    let mut uconf_ids: Vec<i32> = Vec::new();
    for id in CONNECTION.lock().await.borrow().prepare("SELECT hour, minute, user_id FROM config")?.query_map([], |row| {
        let conf_time = mytime::time {
            hour: row.get(0)?,
            minute: row.get(1)?
        };
        if mytime::between_time(&prev, &curr, &conf_time) {
            Ok(row.get(2)?)
        } else {
            Ok(-1)
        }

    })? {
        let i = id.unwrap();
        if i != -1 {
            uconf_ids.push(i);
        }
    }
    Ok(uconf_ids)

}

pub async fn set_db(path: String) -> () {
    CONNECTION.lock().await.replace(Connection::open(path).unwrap());
}
