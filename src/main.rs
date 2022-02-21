use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("symbol", "BTC");
    let client = reqwest::Client::new();

    let resp = client
        .get(" https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        // this section will be commented with generic header
        // TODO find a form to use variable environment or something else to catch the api key
        .header("X-CMC_PRO_API_KEY", "YOUR_API_KEY")
        .query(&params)
        .send()
        .await?;

    let resp = resp.text().await?;

    println!("{:#?}", resp);
    Ok(())
}
