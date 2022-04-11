use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

const DAY_SECONDS: i64 = 24 * 60 * 60;
const CYCLE: i64 = 5;

pub struct Pcr {
    username: String,
    input_time: String,
}

impl Pcr {
    pub fn new(username: String, input_time: String) -> Self {
        Pcr {
            username,
            input_time,
        }
    }

    pub fn calc(&self) -> String {
        let mut last_number = self.username.chars().last().unwrap().to_digit(10).unwrap() as i64;
        last_number = match last_number {
            1..=5 => last_number,
            6..=9 => last_number - 5,
            0 => last_number + 5,
            _ => unreachable!(),
        };

        let input_time = DateTime::<Utc>::from_utc(
            NaiveDateTime::new(
                NaiveDate::parse_from_str(&self.input_time, "%Y-%m-%d")
                    .unwrap_or(NaiveDate::from_ymd(2022, 4, 4)),
                NaiveTime::from_hms(0, 0, 0),
            ),
            Utc,
        );

        let now_time = Utc::now();
        let timestamp = now_time.timestamp() - Utc.ymd(2022, 4, 3).and_hms(16, 0, 0).timestamp();
        let pcr_number = ((timestamp / DAY_SECONDS) % CYCLE) + 1;
        let interval = (pcr_number - last_number + CYCLE) % CYCLE;

        let pcr_time = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(
                std::cmp::max(
                    now_time
                        .timestamp()
                        .checked_sub(interval * DAY_SECONDS)
                        .unwrap(),
                    input_time.timestamp(),
                ),
                0,
            ),
            Utc,
        )
        .with_timezone(&FixedOffset::east(8 * 60 * 60));

        match env::var("ALWAYS_TODAY").unwrap().as_ref() {
            "false" => pcr_time.format("%Y-%m-%d-10").to_string(),
            _ => now_time
                .with_timezone(&FixedOffset::east(8 * 60 * 60))
                .format("%Y-%m-%d-10")
                .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calc() {
        let pcr = Pcr::new("MF1923044".to_string(), "2022-04-10".to_string());
        println!("{}", pcr.calc());
    }
}
