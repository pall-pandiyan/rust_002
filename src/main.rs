use chrono::{Timelike, Utc};

fn print_time() {
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    println!(
        "Time now: {:2}:{:2}:{:2} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    )
}

fn main() {
    print_time();
}
