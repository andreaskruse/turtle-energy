use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;

use crate::Result;

pub struct Point {
    pub ts: i64,
    pub local: DateTime<Tz>,
}

impl Point {
    pub fn new(dt_str: &str, tz: &str) -> Result<Point> {
        let chrono_tz = tz.parse::<Tz>().expect("tz error");
        let naive = NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S")?;
        let utc: DateTime<Utc> = Utc.from_utc_datetime(&naive);
        let local: DateTime<Tz> = utc.with_timezone(&chrono_tz);

        Ok(Point {
            ts: utc.timestamp(),
            local: local,
        })
    }
}
