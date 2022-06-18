use chrono::prelude::{DateTime, Local, Datelike};
use chrono::Duration;


#[derive(Debug)]
pub struct Birthday {
    pub name: String,
    pub day: i32,
    pub month: i32,
    pub reminder: i32,
    pub id: i32,
}

use crate::database;

pub fn sort_bths_name(bths: &mut Vec<Birthday>) -> () {
    bths.sort_by(|a, b| a.name.to_ascii_uppercase().cmp(&b.name.to_ascii_uppercase()));
}

pub fn sort_bths_date(bths: &mut Vec<Birthday>) -> () {
    bths.sort_by(|a, b|{
        compare_bths(a,b)
    });
}

pub fn sort_bths_after_date(bths: &mut Vec<Birthday>, month: i32, day: i32) -> () {
    bths.sort_by(|a, b|{
        let comp_a = compare_bth_date(a, month, day);
        let comp_b = compare_bth_date(b, month, day);
        let comp_ab = compare_bths(a,b);
        if comp_a == comp_b {
            comp_ab
        } else if comp_a == std::cmp::Ordering::Equal {
            std::cmp::Ordering::Less
        } else if comp_b == std::cmp::Ordering::Equal {
            std::cmp::Ordering::Greater
        } else if comp_a == std::cmp::Ordering::Greater {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
}

pub fn compare_bths(a: &Birthday, b: &Birthday) -> std::cmp::Ordering {
    compare_bth_date(a,b.month,b.day)
}
pub fn compare_bth_date(a: &Birthday, month: i32, day: i32) -> std::cmp::Ordering {
    let comp_m = a.month.cmp(&month);
    if comp_m == std::cmp::Ordering::Equal {
        a.day.cmp(&day)
    }
    else {
        comp_m
    }
}

// Put inside struct
pub fn check_bth(a: &Birthday) -> bool {
    match a.month {
        1 => {
            if a.day > 31 {
                return false
            }
        }
        2 => {
            if a.day > 29 {
                return false
            }
        }
        3 => {
            if a.day > 31 {
                return false
            }
        }
        4 => {
            if a.day > 30 {
                return false
            }
        }
        5 => {
            if a.day > 31 {
                return false
            }
        }
        6 => {
            if a.day > 30 {
                return false
            }
        }
        7 => {
            if a.day > 31 {
                return false
            }
        }
        8 => {
            if a.day > 31 {
                return false
            }
        }
        9 => {
            if a.day > 30 {
                return false
            }
        }
        10 => {
            if a.day > 31 {
                return false
            }
        }
        11 => {
            if a.day > 30 {
                return false
            }
        }
        12 => {
            if a.day > 31 {
                return false
            }
        }
        _ => {
            return false
        }
    }
    if a.day < 0 {
        return false
    }

    if a.reminder < 0 {
        return false
    }

    if a.name.is_empty() {
        return false
    }

    true
}

pub async fn get_reminded_birthday(user_id: i32) -> Vec<Birthday> {
    let vec_bth: Vec<Birthday> = database::get_all_birthdays(user_id).await.unwrap();
    let mut output_vec: Vec<Birthday> = Vec::new();
    let today: DateTime<Local> = Local::now();

    for bt in vec_bth {
        let future = today + Duration::days(bt.reminder as i64);
        if compare_bth_date(&bt, today.month() as i32, today.day() as i32) != std::cmp::Ordering::Less && compare_bth_date(&bt, future.month() as i32, future.day() as i32) != std::cmp::Ordering::Greater {
            output_vec.push(bt);
        }
    }

    output_vec
}
