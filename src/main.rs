use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::{collections::HashMap, error::Error};
mod accu;
use accu::AccuProvider;

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    CountryID: String,
    EnglishName: String,
    EnglishType: String,
    ID: String,
    Level: i32,
    LocalizedName: String,
    LocalizedType: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let accu_provider = AccuProvider::new();
    let regions = accu_provider.await.city_search("Ternopil").await;

    // let locations: Vec<Location> = serde_json::from_value(value).unwrap();
    match regions {
        Ok(value) => println!("{:#?}", value),
        Err(e) => println!("{:#?}", e),
    };
    Ok(())
}
