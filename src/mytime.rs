use chrono::prelude::{Local, NaiveTime, Timelike};
use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Debug)]
pub struct time {
    pub hour: i32,
    pub minute: i32,
}

pub fn comp_mytime(a: &time, b: &time) -> std::cmp::Ordering {
    let comp_h = a.hour.cmp(&b.hour);
    if comp_h == Equal {
        a.minute.cmp(&b.minute)
    } else {
        comp_h
    }
}
pub fn between_time(a: &time, b:&time, c:&time) -> bool {
    let comp_ab = comp_mytime(a,b);
    let comp_ac = comp_mytime(a,c);
    let comp_bc = comp_mytime(b,c);

    match comp_ab {
        Less => {
            comp_ac == Less && comp_bc == Greater
        },
        Equal => {
        comp_ac == Equal
        },
        Greater => {
            comp_ac == Less || comp_bc == Greater
        }
    }
}

pub fn get_mytime_from_time() -> time  {
    let ti: NaiveTime = Local::now().time();
    let my = time {
        hour: ti.hour() as i32,
        minute: ti.minute() as i32
    };
    return my
}
