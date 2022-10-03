use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: i64,
    #[serde(rename = "Contact")]
    pub contact: String,
    #[serde(rename = "Salutation/Title")]
    pub salutation_title: String,
    #[serde(rename = "Job Title")]
    pub job_title: String,
    #[serde(rename = "Electorate")]
    pub electorate: String,
    #[serde(rename = "Party")]
    pub party: String,
    #[serde(rename = "Parliament Email")]
    pub parliament_email: String,
}
