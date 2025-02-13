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
enum KnownResultCode {
    #[serde(rename = "successResult")]
    SuccessResult,

    #[serde(rename = "errorResult")]
    GenericError1,

    #[serde(rename = "None")]
    None, // TODO: actually Option::None?

    #[serde(rename = "error")]
    GenericError2,

    #[serde(rename = "VAL01")]
    GenericError3,

    #[serde(rename = "VAL02")]
    TimeBlock1,

    #[serde(rename = "VAL03")]
    AlreadyAddedThisTerm,

    #[serde(rename = "VAL04")]
    NotInCoursePlan,

    #[serde(rename = "VAL05")]
    OverMaximumCreditLimit,

    #[serde(rename = "VAL06")]
    InsufficientQuota1,

    #[serde(rename = "VAL07")]
    PassedBeforeAA,

    #[serde(rename = "VAL08")]
    WrongDegreeProgram,

    #[serde(rename = "VAL09")]
    CourseConflict,

    #[serde(rename = "VAL10")]
    CourseNotRegisteredNoOp,

    #[serde(rename = "VAL11")]
    RequirementsNotMet,

    #[serde(rename = "VAL12")]
    CourseNotOpened,

    #[serde(rename = "VAL13")]
    TemporarilyBlocked,

    #[serde(rename = "VAL14")]
    SystemNoAnswer,

    #[serde(rename = "VAL15")]
    Max12Crn,

    #[serde(rename = "VAL16")]
    ProcessOngoing,

    #[serde(rename = "VAL18")]
    Blocked,

    #[serde(rename = "VAL19")]
    CanNotTakeAssociateCourse,

    #[serde(rename = "VAL20")]
    MustHaveAtLeastOneCourse,

    #[serde(rename = "CRNListEmpty")]
    CrnListEmpty,

    #[serde(rename = "CRNNotFound")]
    CrnNotFound,

    #[serde(rename = "ERRLoad")]
    ErrorLoad,

    #[serde(rename = "NULLParam-CheckOgrenciKayitZamaniKontrolu")]
    TimeBlock2,

    #[serde(rename = "Ekleme İşlemi Başarılı")]
    SuccessfullyAdded,

    #[serde(rename = "Kontenjan Dolu")]
    InsufficientQuota2,

    #[serde(rename = "Silme İşlemi Başarılı")]
    SuccessfullyDropped,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UnknownResultCode(Value);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum ResultCode {
    Known(KnownResultCode),
    Unknown(UnknownResultCode),
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
    result_code: ResultCode,

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
    result_code: ResultCode,

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
