
use std::error::Error;

const API_KEY: &str = "QUp0o9h0Ir30CfhAGsolH5wUDHRUKAbZ";
const URL: &str = "http://dataservice.accuweather.com/locations/v1/adminareas/ua";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = format!("{}?apikey={}", URL, API_KEY);
    let resp = reqwest::get(url).await?.text().await?;
    println!("{:#?}", resp);
    Ok(())
}
