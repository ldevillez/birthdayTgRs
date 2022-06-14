
use std::error::Error;
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use rusqlite::{Connection, Result, params};

type AsyncResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

use crate::bth;

lazy_static! {
    static ref CONNECTION: Mutex<Connection> = Mutex::new(Connection::open("./birthday.db3").unwrap());
}

pub async fn initialize_database() -> AsyncResult<()> {

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

pub async fn get_all_birthdays(user_id: i64) -> AsyncResult<Vec<bth::Birthday>> {
    let mut birthdays: Vec<bth::Birthday> = Vec::new();
    let c = CONNECTION.lock().await;
    let mut statement = c.prepare(&format!("SELECT name, day, month, reminder, id FROM person WHERE user_id ={}", user_id))?;
    let birthdays_map = statement.query_map([], |row| {
        Ok(bth::Birthday {
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

pub async fn create_birthday(user_id: i64, birthday: &bth::Birthday) -> AsyncResult<()> {
    CONNECTION.lock().await.execute("INSERT INTO person (user_id, name, day, month, reminder) VALUES (?1, ?2, ?3, ?4, ?5)", params![user_id, birthday.name, birthday.day, birthday.month, birthday.reminder],)?;

    Ok(())
}

pub async fn delete_birthday(user_id: i64, id: i64) -> AsyncResult<()>{
    CONNECTION.lock().await.execute("DELETE FROM person WHERE id=?1 AND user_id=?2", params![id,user_id],)?;

    Ok(())
}

pub async fn edit_birthday(user_id: i64, birthday: &bth::Birthday) -> AsyncResult<()>{
    CONNECTION.lock().await.execute("UPDATE person SET name = ?1, day = ?2, month = ?3, reminder=?4 WHERE id = ?5 AND user_id = ?6", params![birthday.name, birthday.day, birthday.month, birthday.reminder, birthday.id,user_id],)?;

    Ok(())
}
