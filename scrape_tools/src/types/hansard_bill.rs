use std::fmt::{Display, Formatter, Result};

pub struct HansardBill {
    pub name: String,
    pub mpInCharge: String,
    pub stage: Stage,
    pub reports: Vec<HansardReport>,
}
pub struct HansardReport {
    pub name: String,
    pub report_type: ReportType,
    pub link: String,
}
#[derive(Debug)]
pub enum ReportType {
    InCommittee,
    FirstReading,
    SecondReading,
    ThirdReading,
    Unknown(String),
}
impl Display for ReportType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
pub enum Stage {
    Introduction,
    FirstReading,
    SelectCommittee,
    SecondReading,
    CommitteOfWholeHouse,
    ThirdReading,
    RoyalAssent,
    Abandoned,
}
