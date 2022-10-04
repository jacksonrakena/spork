use async_trait::async_trait;

use crate::types::{mp_request_response::MpRequestResponse, parliament_member::Member};

use super::scraper::{ResourceScraper, ScrapeError};

pub struct MemberScraper {}

#[async_trait]
impl ResourceScraper<Member> for MemberScraper {
    async fn scrape(&self) -> Result<Vec<Member>, ScrapeError> {
        let client = reqwest::Client::new();
        let resource_id = "89069a40-abcf-4190-9665-3513ff004dd8";

        match client
            .get(
                "https://catalogue.data.govt.nz/api/3/action/datastore_search?resource_id="
                    .to_owned()
                    + resource_id
                    + "&limit=200",
            )
            .send()
            .await
        {
            Ok(string_response) => match string_response.text().await {
                Ok(string_response_text) => {
                    match serde_json::from_str::<MpRequestResponse>(
                        &string_response_text.to_owned(),
                    ) {
                        Ok(response) => Ok(response.result.records),
                        Err(_) => Err(ScrapeError::ParseFail),
                    }
                }
                Err(_) => Err(ScrapeError::RequestFail),
            },
            Err(_) => Err(ScrapeError::RequestFail),
        }
    }
}
