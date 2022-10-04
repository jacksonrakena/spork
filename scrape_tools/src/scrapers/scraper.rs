use std::fmt::{Display, Formatter};

use async_trait::async_trait;

#[derive(Debug)]
pub enum ScrapeError {
    RequestFail,
    ParseFail,
}

impl Display for ScrapeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[async_trait]
pub trait ResourceScraper<T> {
    async fn scrape(&self) -> Result<Vec<T>, ScrapeError>;
}

#[async_trait]
pub trait ResourceScraperQueryable<T> {
    async fn scrape_query(&self, query: &String) -> Result<T, ScrapeError>;
}
