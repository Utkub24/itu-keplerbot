use std::fmt::Display;

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

// Kaynak: https://github.com/AtaTrkgl/itu-ders-secici, https://github.com/MustafaKrc/ITU-CRN-Picker <3
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

impl KnownResultCode {
    pub fn description(&self) -> &'static str {
        match self {
            KnownResultCode::SuccessResult => "İşlem başarıyla tamamlandı.",
            KnownResultCode::GenericError1 | KnownResultCode::None => "Operasyon tamamlanamadı.",
            KnownResultCode::GenericError2 => "Bir hata meydana geldi.",
            KnownResultCode::GenericError3 => "Bir problemden dolayı alınamadı",
            KnownResultCode::TimeBlock1 | KnownResultCode::TimeBlock2 => {
                "Kayıt zaman engelinden dolayı alınamadı"
            }
            KnownResultCode::AlreadyAddedThisTerm => {
                "Bu dönem zaten alındığından dolayı tekrar alınmadı."
            }
            KnownResultCode::NotInCoursePlan => "Ders planında yer almadığından dolayı alınamadı.",
            KnownResultCode::OverMaximumCreditLimit => {
                "Dönemlik maksimum kredi sınırını aştığından dolayı alınamadı."
            }
            KnownResultCode::InsufficientQuota1 | KnownResultCode::InsufficientQuota2 => {
                "Kontenjan yetersizliğinden dolayı alınamadı."
            }
            KnownResultCode::PassedBeforeAA => {
                "Daha önce AA notuyla verildiğinden dolayı alınamadı."
            }
            KnownResultCode::WrongDegreeProgram => {
                "Program şartını sağlamadığından dolayı alınamadı."
            }
            KnownResultCode::CourseConflict => "Başka bir dersle çakıştığından dolayı alınamadı.",
            KnownResultCode::CourseNotRegisteredNoOp => {
                "Derse kayıtlı olmadığınızdan dolayı hiç bir işlem yapılmadı."
            }
            KnownResultCode::RequirementsNotMet => "Önşartlardan dolayı alınamadı.",
            KnownResultCode::CourseNotOpened => {
                "Şu anki dönemde hiç açılmadığından dolayı alınamadı."
            }
            KnownResultCode::TemporarilyBlocked => {
                "Geçici olarak engellenmiş olması sebebiyle alınamadı."
            }
            KnownResultCode::SystemNoAnswer | KnownResultCode::ErrorLoad => {
                "Sistem geçici olarak yanıt vermiyor."
            }
            KnownResultCode::Max12Crn => "Maksimum 12 CRN alabilirsiniz,",
            KnownResultCode::ProcessOngoing => {
                "Aktif bir işleminiz devam ettiğinden dolayı işlem yapılmadı."
            }
            KnownResultCode::Blocked => "Engellendiğinden dolayı alınamadı.",
            KnownResultCode::CanNotTakeAssociateCourse => {
                "Önlisans dersi olduğundan dolayı alınamadı."
            }
            KnownResultCode::MustHaveAtLeastOneCourse => {
                "Dönem başına sadece 1 ders bırakabilirsiniz."
            }
            KnownResultCode::CrnListEmpty => "CRN listesi boş göründüğünden alınamadı.",
            KnownResultCode::CrnNotFound => "CRN bulunamadığından dolayı alınamadı.",
            KnownResultCode::SuccessfullyAdded => "Ekleme işlemi başarıyla tamamlandı.",
            KnownResultCode::SuccessfullyDropped => "Silme işlemi başarıyla tamamlandı.",
        }
    }
}

impl Display for KnownResultCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UnknownResultCode(Value);

impl Display for UnknownResultCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bilinmeyen Sonuç ({})", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum ResultCode {
    Known(KnownResultCode),
    Unknown(UnknownResultCode),
}

impl Display for ResultCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultCode::Known(k) => write!(f, "{}", k),
            ResultCode::Unknown(u) => write!(f, "{}", u),
        }
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
    result_code: ResultCode,

    #[serde(rename = "resultData")]
    result_data: Option<ResultData>,
}

impl Display for CrnAddResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CRN {}: {}", self.crn, self.result_code)
    }
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

impl Display for CrnDropResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CRN {}: {}", self.crn, self.result_code)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourseSelectionResponseBody {
    #[serde(rename = "ecrnResultList")]
    ecrn_result_list: Vec<CrnAddResult>,

    #[serde(rename = "scrnResultList")]
    scrn_result_list: Vec<CrnDropResult>,
}

impl Display for CourseSelectionResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Eklenen CRN Sonuçları")?;
        self.ecrn_result_list
            .iter()
            .try_for_each(|e| writeln!(f, "{}", e))?;

        writeln!(f, "Çıkarılan CRN Sonuçları")?;
        self.scrn_result_list
            .iter()
            .try_for_each(|e| writeln!(f, "{}", e))?;

        Ok(())
    }
}
