pub struct HansardBill {
    pub name: String,
    pub mpInCharge: String,
    pub stage: Stage,
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
