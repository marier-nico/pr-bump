use chrono::{DateTime, TimeZone, Utc};

pub fn ymd_midnight(year: i32, month: u32, day: u32) -> DateTime<Utc> {
    Utc.ymd(year, month, day).and_hms(0, 0, 0)
}
