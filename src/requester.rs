use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use reqwest::{
    cookie::{CookieStore, Jar},
    Client, Request, RequestBuilder, Response, StatusCode, Url,
};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Debug, ops::Deref, str::FromStr, sync::Arc, thread};

use crate::cli::MakeConfigArgs;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CourseSelectionRequestBody {
    #[serde(rename = "ECRN")]
    crn_list: Vec<String>,

    #[serde(rename = "SCRN")]
    scrn_list: Vec<String>,
}

impl CourseSelectionRequestBody {
    pub fn new(crn_list: Vec<String>, scrn_list: Vec<String>) -> Self {
        Self {
            crn_list,
            scrn_list,
        }
    }
}

impl From<Config> for CourseSelectionRequestBody {
    fn from(config: Config) -> Self {
        Self::new(config.crn_list, config.scrn_list)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LoginFormHiddenFields {
    #[serde(rename = "__EVENTTARGET")]
    event_target: String,

    #[serde(rename = "__EVENTARGUMENT")]
    event_argument: String,

    #[serde(rename = "__VIEWSTATE")]
    viewstate: String,

    #[serde(rename = "__VIEWSTATEGENERATOR")]
    viewstate_generator: String,

    #[serde(rename = "__EVENTVALIDATION")]
    event_validation: String,
}

impl LoginFormHiddenFields {
    const EVENT_TARGET_NAME: &str = "__EVENTTARGET";
    const EVENT_ARGUMENT_NAME: &str = "__EVENTARGUMENT";
    const VIEWSTATE_NAME: &str = "__VIEWSTATE";
    const VIEWSTATE_GENERATOR_NAME: &str = "__VIEWSTATEGENERATOR";
    const EVENT_VALIDATION_NAME: &str = "__EVENTVALIDATION";

    pub fn new(
        event_target: String,
        event_argument: String,
        viewstate: String,
        viewstate_generator: String,
        event_validation: String,
    ) -> Self {
        Self {
            event_target,
            event_argument,
            viewstate,
            viewstate_generator,
            event_validation,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LoginFormInputFields {
    #[serde(rename = "ctl00$ContentPlaceHolder1$hfAppName")]
    app_name: String,

    #[serde(rename = "ctl00$ContentPlaceHolder1$hfToken")]
    hf_token: String,

    #[serde(rename = "ctl00$ContentPlaceHolder1$hfVerifier")]
    hf_verifier: String,

    #[serde(rename = "ctl00$ContentPlaceHolder1$hfCode")]
    hf_code: String,

    #[serde(rename = "ctl00$ContentPlaceHolder1$hfState")]
    hf_state: String,

    #[serde(rename = "ctl00$ContentPlaceHolder1$tbUserName")]
    username: String,

    #[serde(rename = "ctl00$ContentPlaceHolder1$tbPassword")]
    password: String,

    #[serde(rename = "ctl00$ContentPlaceHolder1$btnLogin")]
    btn_login: String,
}

impl LoginFormInputFields {
    const APP_NAME: &str = "İTÜ/Portal";
    const HF_TOKEN: &str = "";
    const HF_VERIFIER: &str = "";
    const HF_CODE: &str = "";
    const HF_STATE: &str = "";
    const BTN_LOGIN: &str = "Giriş+/+Login";

    pub fn new(username: String, password: String) -> Self {
        Self {
            app_name: Self::APP_NAME.into(),
            hf_token: Self::HF_TOKEN.into(),
            hf_verifier: Self::HF_VERIFIER.into(),
            hf_code: Self::HF_CODE.into(),
            hf_state: Self::HF_STATE.into(),
            username,
            password,
            btn_login: Self::BTN_LOGIN.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LoginFormBody {
    #[serde(flatten)]
    hidden_fields: LoginFormHiddenFields,

    #[serde(flatten)]
    input_fields: LoginFormInputFields,
}

impl LoginFormBody {
    pub fn new(hidden_fields: LoginFormHiddenFields, input_fields: LoginFormInputFields) -> Self {
        Self {
            hidden_fields,
            input_fields,
        }
    }
}

impl From<Config> for LoginFormInputFields {
    fn from(config: Config) -> Self {
        Self::new(config.username, config.password)
    }
}

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
    jar: Arc<Jar>,
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
        let jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_provider(Arc::clone(&jar))
            .build()
            .expect("Client::builder()");
        Self {
            config,
            client,
            jar: Arc::clone(&jar),
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

        println!("API Token alınıyor...");
        let jwt = self.fetch_jwt().await?;
        println!("API Token başarılı bir şekilde alındı");
        let request = self.build_course_selection_request(&jwt)?;

        println!("Ders seçiliyor...");
        let res = self.send_request(&request).await?;
        println!("{:?}", res);
        let text = res.text().await?;
        println!("{}", text);

        Ok(())
    }

    fn build_login_form_hidden_fields(document: Html) -> LoginFormHiddenFields {
        let event_target_selector = Selector::parse("input[name='__EVENTTARGET']").unwrap();
        let event_target = document
            .select(&event_target_selector)
            .next()
            .and_then(|el| el.value().attr("value"))
            .unwrap_or_default();

        let event_argument_selector = Selector::parse("input[name='__EVENTARGUMENT']").unwrap();
        let event_argument = document
            .select(&event_argument_selector)
            .next()
            .and_then(|el| el.value().attr("value"))
            .unwrap_or_default();

        let viewstate_selector = Selector::parse("input[name='__VIEWSTATE']").unwrap();
        let viewstate = document
            .select(&viewstate_selector)
            .next()
            .and_then(|el| el.value().attr("value"))
            .unwrap_or_default();

        let viewstate_generator_selector =
            Selector::parse("input[name='__VIEWSTATEGENERATOR']").unwrap();
        let viewstate_generator = document
            .select(&viewstate_generator_selector)
            .next()
            .and_then(|el| el.value().attr("value"))
            .unwrap_or_default();

        let event_validation_selector = Selector::parse("input[name='__EVENTVALIDATION']").unwrap();
        let event_validation = document
            .select(&event_validation_selector)
            .next()
            .and_then(|el| el.value().attr("value"))
            .unwrap_or_default();

        LoginFormHiddenFields::new(
            event_target.to_string(),
            event_argument.to_string(),
            viewstate.to_string(),
            viewstate_generator.to_string(),
            event_validation.to_string(),
        )
    }

    fn build_login_form(document: Html, config: Config) -> LoginFormBody {
        let hidden_fields = Self::build_login_form_hidden_fields(document);
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
        self.login().await?; // TODO: check if already logged in (?)

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
