use chrono::{Datelike, Timelike, Utc};

fn print_datetime() {
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    println!(
        "Datetime now: {}/{}/{} {:2}:{:2}:{:2} {}",
        now.day(),
        now.month(),
        now.year(),
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    )
}

fn main() {
    print_datetime();
}
