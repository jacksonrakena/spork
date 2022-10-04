use async_trait::async_trait;

use crate::types::hansard_bill::{HansardBill, HansardReport, ReportType, Stage};

use super::scraper::{ResourceScraperQueryable, ScrapeError};

pub struct BillScraper {}
#[async_trait]
impl ResourceScraperQueryable<HansardBill> for BillScraper {
    async fn scrape_query(&self, query: &String) -> Result<HansardBill, ScrapeError> {
        let url = "https://www.parliament.nz/en/pb/bills-and-laws/bills-proposed-laws/document/"
            .to_owned()
            + query
            + "/tab/hansard";

        let client = reqwest::Client::new();
        match client.get(url).send().await {
            Ok(response) => match response.text().await {
                Ok(response_text) => parse_bill(response_text),
                Err(_) => Err(ScrapeError::ParseFail),
            },
            Err(_) => Err(ScrapeError::RequestFail),
        }
    }
}

fn parse_bill(text: String) -> Result<HansardBill, ScrapeError> {
    match tl::parse(&text, tl::ParserOptions::default()) {
        Ok(dom) => {
            let parser = dom.parser();
            let mut bill = HansardBill {
                name: "".to_string(),
                mpInCharge: "".to_string(),
                stage: Stage::Introduction,
                reports: vec![],
            };

            let elements = dom.get_elements_by_class_name("list__cell-heading");
            for element in elements {
                let real_element = element.get(parser).unwrap();
                let href = real_element
                    .as_tag()
                    .unwrap()
                    .attributes()
                    .get("href")
                    .unwrap()
                    .unwrap()
                    .as_utf8_str()
                    .to_string();
                let name_element = real_element.inner_text(parser);
                let name = name_element.trim();
                bill.reports.push(HansardReport {
                    name: name.to_owned(),
                    link: href.to_owned(),
                    report_type: try_determine_report_type(name),
                });
            }
            Ok(bill)
        }
        Err(_) => Err(ScrapeError::ParseFail),
    }
}

fn try_determine_report_type(content: &str) -> ReportType {
    if content.contains("In Committee") {
        ReportType::InCommittee
    } else if content.contains("First Reading") {
        ReportType::FirstReading
    } else if content.contains("Second Reading") {
        ReportType::SecondReading
    } else if content.contains("Third Reading") {
        ReportType::ThirdReading
    } else {
        ReportType::Unknown(content.to_owned())
    }
}
