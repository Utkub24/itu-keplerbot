use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use humantime::format_duration;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    time::{Duration, SystemTime},
};

use crate::cli::MakeConfigArgs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub time: chrono::DateTime<FixedOffset>,
}

impl Config {
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
        const TRT_OFFSET_SECONDS: i64 = 3 * 3600; // UTC+3 TRT
        let offset = FixedOffset::east_opt(TRT_OFFSET_SECONDS as i32).unwrap();
        let time = DateTime::from_timestamp(
            value.time.as_secs() as i64 - TRT_OFFSET_SECONDS,
            value.time.subsec_nanos(),
        )
        .unwrap()
        .with_timezone(&offset);

        Config::new(value.username, value.password, time)
    }
}

#[derive(Debug)]
pub struct Requester {
    config: Config,
}

impl Requester {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn say_hello(&self) {
        println!("hello!");
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let now = SystemTime::now();
        println!(
            "Şuan saat {}",
            format_duration(now.duration_since(SystemTime::UNIX_EPOCH)?)
        );

        println!();

        Ok(())
    }
}
