use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use std::{error::Error, thread};

use crate::cli::MakeConfigArgs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub time: chrono::DateTime<FixedOffset>,
}

impl Config {
    const TRT_OFFSET_SECONDS: i64 = 3 * 3600; // UTC+3 TRT
    const TRT_TIMEZONE: FixedOffset = FixedOffset::east_opt(Self::TRT_OFFSET_SECONDS as i32)
        .expect("TRT Timezone should not fail");
    pub fn new(username: String, password: String, time: chrono::DateTime<FixedOffset>) -> Self {
        Self {
            username,
            password,
            time,
        }
    }
}

impl From<MakeConfigArgs> for Config {
    fn from(value: MakeConfigArgs) -> Self {
        // ASSUME INPUT IS IN UTC+3 TRT TIME
        let time = DateTime::from_timestamp(
            value.time.as_secs() as i64 - Config::TRT_OFFSET_SECONDS,
            value.time.subsec_nanos(),
        )
        .unwrap()
        .with_timezone(&Config::TRT_TIMEZONE);

        Config::new(value.username, value.password, time)
    }
}

#[derive(Debug)]
pub struct Requester {
    config: Config,
}

fn now_trt() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&Config::TRT_TIMEZONE)
}

fn print_time_trt() {
    println!("Şuan saat {}", now_trt());
}

impl Requester {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        print_time_trt();
        let now = now_trt();
        let until = self.config.time.signed_duration_since(now);
        println!("Ders seçimine {} var", until);

        const ONE_MINUTE_DELTA: TimeDelta =
            TimeDelta::new(60, 0).expect("one minute delta should not fail");
        let sleep_time = until - ONE_MINUTE_DELTA;

        match sleep_time.to_std() {
            Ok(sleep_time) => {
                println!("Ders seçimine 1 dakika kalana kadar bekleniyor...");
                thread::sleep(sleep_time);
            }
            Err(_) => println!("Ders seçimine 1 dakikadan az var"),
        }

        print_time_trt();

        println!("Mock Fetch JWT");

        Ok(())
    }
}
