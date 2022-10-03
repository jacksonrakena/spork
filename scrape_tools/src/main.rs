use crate::types::mp_request_response::MpRequestResponse;

mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    scrape_all_members().await;
    Ok(())
}

async fn scrape_all_members() {
    let client = reqwest::Client::new();
    let resource_id = "89069a40-abcf-4190-9665-3513ff004dd8";

    match client
        .get(
            "https://catalogue.data.govt.nz/api/3/action/datastore_search?resource_id=".to_owned()
                + resource_id
                + "&limit=200",
        )
        .send()
        .await
    {
        Ok(string_response) => {
            let string_response_text = string_response.text().await.unwrap();
            let response: MpRequestResponse =
                serde_json::from_str(&string_response_text.to_owned()).unwrap();
            println!(
                "Downloaded {} members of Parliament",
                response.result.records.len()
            );
        }
        Err(_) => panic!("Failed to download members."),
    }
}
