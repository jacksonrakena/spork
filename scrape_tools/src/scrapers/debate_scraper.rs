use async_trait::async_trait;
use tl::NodeHandle;

use crate::types::hansard_bill::HansardBill;

use super::scraper::{ResourceScraperQueryable, ScrapeError};

pub struct DebateScraper {}
pub struct HansardDebateSpeech {
    speaker: String,
    text: String,
}
pub struct HansardDebate {
    speeches: Vec<HansardDebateSpeech>,
}

#[async_trait]
impl ResourceScraperQueryable<HansardDebate> for DebateScraper {
    async fn scrape_query(&self, query: &String) -> Result<HansardDebate, ScrapeError> {
        let url = "https://www.parliament.nz/".to_owned() + query;

        let client = reqwest::Client::new();
        match client.get(url).send().await {
            Ok(response) => match response.text().await {
                Ok(response_text) => parse_debate(response_text),
                Err(_) => Err(ScrapeError::ParseFail),
            },
            Err(_) => Err(ScrapeError::RequestFail),
        }
    }
}

fn parse_debate(text: String) -> Result<HansardDebate, ScrapeError> {
    match tl::parse(&text, tl::ParserOptions::default()) {
        Ok(dom) => {
            let parser = dom.parser();
            let mut debate = HansardDebate { speeches: vec![] };

            for speech_element in dom.get_elements_by_class_name("Speech") {
                println!(
                    "Speech {}",
                    speech_element
                        .get(parser)
                        .unwrap()
                        .inner_text(parser)
                        .to_owned()
                )
            }
            Ok(debate)
        }
        Err(_) => Err(ScrapeError::ParseFail),
    }
}
