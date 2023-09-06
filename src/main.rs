let tenant_id = String::from("d91b8372-e1d3-4a61-a6be-2f532e44d264");
let client_id = String::from("5222cf47-495f-40f1-ab3d-aaf649d2fed5");
let client_secret = String::from("8287359e-044c-4c3d-9cc3-11b11ffa7053");


use azure_core::auth::TokenCredential;
use azure_identity::{DefaultAzureCredential};
use url::Url;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credential = DefaultAzureCredential::default();
    let response = credential
        .get_token("https://management.azure.com")
        .await?;

    let subscription_id = env::var("AZURE_SUBSCRIPTION_ID")?;
    let url = Url::parse(&format!(
        "https://management.azure.com/subscriptions/{}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01",
        subscription_id))?;
    let response = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", response.token.secret()))
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", response);
    Ok(())
}
