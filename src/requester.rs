use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use reqwest::{Client, Request, RequestBuilder, Response};
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::{error::Error, thread};

use crate::{
    cli::MakeConfigArgs,
    course::{CourseSelectionRequestBody, CourseSelectionResponseBody},
    login::{LoginFormBody, LoginFormHiddenFields, LoginFormInputFields},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    const COURSE_SELECT_URL: &str = "https://obs.itu.edu.tr/api/ders-kayit/v21";
    const LOGIN_URL: &str = "https://girisv3.itu.edu.tr";
    const FETCH_JWT_URL: &str = "https://obs.itu.edu.tr/ogrenci/auth/jwt";

    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .expect("Client::builder()");
        Self { config, client }
    }

    fn until(&self) -> TimeDelta {
        let now = now_trt();
        self.config.time.signed_duration_since(now)
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let until = self.until();
        println!("Ders seçimine {} var", until);

        const ONE_MINUTE_DELTA: TimeDelta =
            TimeDelta::new(60, 0).expect("one minute delta should not fail");
        let sleep_time = until - ONE_MINUTE_DELTA;

        match sleep_time.to_std() {
            Ok(sleep_time) => {
                println!("Ders seçimine 1 dakika kalana kadar bekleniyor...");
                thread::sleep(sleep_time);
            }
            Err(_) => println!("Ders seçimine 1 dakikadan az var, program başlatılıyor..."),
        }

        print_time_trt();
        println!("Kepler'e giriş yapılıyor...");

        self.login().await?;

        println!("Kepler'e giriş başarılı!");

        println!("API Token alınıyor...");

        let jwt = self.fetch_jwt().await?;

        println!("API Token başarılı bir şekilde alındı!");

        let until = self.until();
        println!("Ders seçimine {} var", until);

        const ERROR_MARGIN: TimeDelta = TimeDelta::new(0, 1000000).expect("1ms");
        let sleep_time = until + ERROR_MARGIN;

        match sleep_time.to_std() {
            Ok(sleep_time) => {
                println!("Ders seçimine kadar bekleniyor...");
                thread::sleep(sleep_time);
            }
            Err(_) => println!("Ders seçimi başlamış! Program başlatılıyor..."),
        }

        println!("Ders seçiliyor...");

        let request = self.build_course_selection_request(&jwt)?;

        const TRY_COUNT: u64 = 100;
        println!("{} kere denenecek.", TRY_COUNT);

        for i in 1..=TRY_COUNT {
            let res = self.send_request(&request).await?;

            let res_body: CourseSelectionResponseBody =
                serde_json::from_slice(&res.bytes().await?)?;

            println!("{}. Deneme", i);
            println!("{}", res_body);
        }

        Ok(())
    }

    fn build_login_form(document: Html, config: Config) -> LoginFormBody {
        let hidden_fields = LoginFormHiddenFields::from(document);
        let input_fields = LoginFormInputFields::from(config);

        LoginFormBody::new(hidden_fields, input_fields)
    }

    async fn login(&self) -> Result<Response, reqwest::Error> {
        let login_page_res = self.client.get(Self::LOGIN_URL).send().await?;
        let referer_url = login_page_res.url().to_string();
        let body = login_page_res.text().await?;
        let document = Html::parse_document(&body);

        let login_form = Self::build_login_form(document, self.config.clone());

        Ok(self
            .client
            .post(&referer_url)
            .header("Referer", referer_url)
            .form(&login_form)
            .send()
            .await?)
    }

    async fn fetch_jwt(&self) -> Result<String, reqwest::Error> {
        // first request sets cookies
        let _ = self.client.get(Self::FETCH_JWT_URL).send().await?;

        // second requests fetches JWT
        let res = self.client.get(Self::FETCH_JWT_URL).send().await?;

        Ok(res.text().await?)
    }

    fn build_course_selection_request(&self, jwt: &str) -> reqwest::Result<Request> {
        let request_body = CourseSelectionRequestBody::from(self.config.clone());

        self.client
            .post(Self::COURSE_SELECT_URL)
            .bearer_auth(jwt)
            .json(&request_body)
            .build()
    }

    async fn send_request(&self, request: &Request) -> Result<Response, reqwest::Error> {
        RequestBuilder::from_parts(self.client.clone(), request.try_clone().unwrap())
            .send()
            .await
    }
}
