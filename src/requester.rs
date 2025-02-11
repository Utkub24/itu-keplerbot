use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::cli::MakeConfigArgs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub time: chrono::DateTime<FixedOffset>,
}

impl Config {
    const TRT_OFFSET_SECONDS: i64 = 3 * 3600; // UTC+3 TRT
    const TIMEZONE: FixedOffset = FixedOffset::east_opt(Self::TRT_OFFSET_SECONDS as i32)
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
        .with_timezone(&Config::TIMEZONE);

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

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let now = Utc::now().with_timezone(&Config::TIMEZONE);
        println!("Şuan saat {}", now);

        println!();

        Ok(())
    }
}
