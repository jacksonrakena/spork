use crate::types::hansard_bill::{HansardBill, Stage};

pub async fn scrape_bill_by_id(id: &String) -> Result<HansardBill, &'static str> {
    let url = "https://www.parliament.nz/en/pb/bills-and-laws/bills-proposed-laws/document/"
        .to_owned()
        + id
        + "/tab/hansard";

    let client = reqwest::Client::new();
    match client.get(url).send().await {
        Ok(response) => match response.text().await {
            Ok(response_text) => {
                let bill = HansardBill {
                    name: "".to_string(),
                    mpInCharge: "".to_string(),
                    stage: Stage::Introduction,
                };
                Ok(bill)
            }
            Err(_) => Err("Failed to request bill data."),
        },
        Err(_) => Err("Failed to request bill data."),
    }
}
