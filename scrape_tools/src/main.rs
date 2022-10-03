use crate::scrapers::mp_scraper::scrape_all_members;

mod scrapers;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    scrape_all_members().await;
    Ok(())
}
