use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::requester::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginFormHiddenFields {
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

fn get_field_value_attr(document: &Html, selector: &Selector) -> String {
    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("value"))
        .unwrap_or_default()
        .into()
}

impl From<Html> for LoginFormHiddenFields {
    fn from(document: Html) -> Self {
        let event_target_selector =
            Selector::parse("input[name='__EVENTTARGET']").expect("__EVENTTARGET selector");
        let event_target = get_field_value_attr(&document, &event_target_selector);

        let event_argument_selector =
            Selector::parse("input[name='__EVENTARGUMENT']").expect("__EVENTARGUMENT selector");
        let event_argument = get_field_value_attr(&document, &event_argument_selector);

        let viewstate_selector =
            Selector::parse("input[name='__VIEWSTATE']").expect("__VIEWSTATE selector");
        let viewstate = get_field_value_attr(&document, &viewstate_selector);

        let viewstate_generator_selector = Selector::parse("input[name='__VIEWSTATEGENERATOR']")
            .expect("__VIEWSTATEGENERATOR selector");
        let viewstate_generator = get_field_value_attr(&document, &viewstate_generator_selector);

        let event_validation_selector =
            Selector::parse("input[name='__EVENTVALIDATION']").expect("__EVENTVALIDATION selector");
        let event_validation = get_field_value_attr(&document, &event_validation_selector);

        LoginFormHiddenFields::new(
            event_target,
            event_argument,
            viewstate,
            viewstate_generator,
            event_validation,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginFormInputFields {
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
pub struct LoginFormBody {
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
