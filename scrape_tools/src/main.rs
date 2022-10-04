use crate::scrapers::bill_scraper::scrape_bill_by_id;
use crate::scrapers::mp_scraper::scrape_all_members;

mod scrapers;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    scrape_all_members().await;
    scrape_bill_by_id("BILL_118526".to_string()).await;
    Ok(())
}
