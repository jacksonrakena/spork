use crate::scrapers::bill_scraper::BillScraper;
use crate::scrapers::debate_scraper::DebateScraper;
use crate::scrapers::mp_scraper::MemberScraper;
use crate::scrapers::scraper::ResourceScraper;
use crate::scrapers::scraper::ResourceScraperQueryable;
use crate::scrapers::scraper::ScrapeError;

mod scrapers;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let mp = MemberScraper {};
    let bill = BillScraper {};
    let debates = DebateScraper {};
    let mps = mp.scrape().await.unwrap();
    println!("scraped {} members", mps.len());
    let bill = bill.scrape_query(&"BILL_112194".to_string()).await.unwrap();
    println!("scraped bill with name {}", bill.name);
    for report in bill.reports {
        let scraped_report = debates.scrape_query(&report.link).await.unwrap();
    }
    Ok(())
}
