use serde::Deserialize;
use serde::Serialize;

use super::parliament_member::Member;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MpRequestResponse {
    pub help: String,
    pub success: bool,
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    #[serde(rename = "include_total")]
    pub include_total: bool,
    pub limit: i64,
    #[serde(rename = "records_format")]
    pub records_format: String,
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    pub records: Vec<Member>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub start: String,
    pub next: String,
}
