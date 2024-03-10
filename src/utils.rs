pub fn phone_validator(phone: &str) -> bool {
    if phone.len() != 11 || !phone.starts_with("09") {
        return false;
    }

    if phone.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    true
}

pub fn now() -> i64 {
    chrono::Local::now().timestamp()
}
