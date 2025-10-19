use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use crate::Result;

pub struct Point {
    pub ts: i64,
    pub utc: DateTime<Utc>,
    pub local: DateTime<Tz>,
}

impl Point {
    pub fn new(dt_str: &str, tz: &str) -> Result<Point> {
        let chrono_tz = tz.parse::<Tz>()?;
        let utc: DateTime<Utc> = dt_str.parse()?;
        let local: DateTime<Tz> = utc.with_timezone(&chrono_tz);

        Ok(Point {
            ts: utc.timestamp(),
            utc: utc,
            local: local,
        })
    }
}
