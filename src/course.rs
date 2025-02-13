use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::requester::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourseSelectionRequestBody {
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
struct ResultData(Value); // actual type unknown, accept any

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CrnAddResult {
    crn: String,

    #[serde(rename = "operationFinished")]
    operation_finished: bool,

    #[serde(rename = "statusCode")]
    status_code: i64,

    #[serde(rename = "resultCode")]
    result_code: String,

    #[serde(rename = "resultData")]
    result_data: Option<ResultData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CrnDropResult {
    crn: String,

    #[serde(rename = "operationFinished")]
    operation_finished: bool,

    #[serde(rename = "statusCode")]
    status_code: i64,

    #[serde(rename = "resultCode")]
    result_code: String,

    #[serde(rename = "resultData")]
    result_data: Option<ResultData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourseSelectionResponseBody {
    #[serde(rename = "ecrnResultList")]
    ecrn_result_list: Vec<CrnAddResult>,

    #[serde(rename = "scrnResultList")]
    scrn_result_list: Vec<CrnDropResult>,
}
