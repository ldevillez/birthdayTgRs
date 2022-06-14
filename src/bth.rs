#[derive(Debug)]
pub struct Birthday {
    pub name: String,
    pub day: i32,
    pub month: i32,
    pub reminder: i32,
    pub id: i32,
}

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
