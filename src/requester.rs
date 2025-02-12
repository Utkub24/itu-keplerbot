use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use reqwest::{Client, Request, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Debug, thread};

use crate::cli::MakeConfigArgs;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    ECRN: Vec<String>,
    SCRN: Vec<String>,
}

impl RequestBody {
    pub fn new(crn_list: Vec<String>, scrn_list: Vec<String>) -> Self {
        Self {
            ECRN: crn_list,
            SCRN: scrn_list,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub time: chrono::DateTime<FixedOffset>,
    pub crn_list: Vec<String>,
    pub scrn_list: Vec<String>,
}

impl Config {
    const TRT_OFFSET_SECONDS: i64 = 3 * 3600; // UTC+3 TRT
    const TRT_TIMEZONE: FixedOffset = FixedOffset::east_opt(Self::TRT_OFFSET_SECONDS as i32)
        .expect("TRT Timezone should not fail");
    pub fn new(
        username: String,
        password: String,
        time: chrono::DateTime<FixedOffset>,
        crn_list: Vec<String>,
        scrn_list: Vec<String>,
    ) -> Self {
        Self {
            username,
            password,
            time,
            crn_list,
            scrn_list,
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

        Config::new(
            value.username,
            value.password,
            time,
            value.crn_list,
            value.scrn_list,
        )
    }
}

#[derive(Debug)]
pub struct Requester {
    config: Config,
    client: Client,
}

fn now_trt() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&Config::TRT_TIMEZONE)
}

fn print_time_trt() {
    println!("Şuan saat {}", now_trt());
}

impl Requester {
    const COURSE_SELECT_URL: &str = "https://obs.itu.edu.tr/api/ders-kayit/v21/";

    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
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
        let jwt = "JWT";

        let request = self.build_course_selection_request(jwt)?;
        println!("Sending request...");
        let res = self.send_request(&request).await?;
        println!("{:?}", res);

        Ok(())
    }

    fn build_course_selection_request(&self, jwt: &str) -> Result<Request, Box<dyn Error>> {
        let request_body =
            RequestBody::new(self.config.crn_list.clone(), self.config.scrn_list.clone());

        Ok(self
            .client
            .post(Self::COURSE_SELECT_URL)
            .bearer_auth(jwt)
            .json(&request_body)
            .build()?)
    }

    async fn send_request(&self, request: &Request) -> Result<Response, reqwest::Error> {
        RequestBuilder::from_parts(self.client.clone(), request.try_clone().unwrap())
            .send()
            .await
    }
}
