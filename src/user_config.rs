
#[derive(Debug)]
pub struct UserConfig {
    pub hour: i32,
    pub minute: i32,
}



// Put inside struct
pub fn check_user_config(a: &UserConfig) -> bool {
    if a.hour < 0 || a.hour > 23{
        return false
    }
    if a.minute < 0 || a.minute > 59{
        return false
    }

    true
}
