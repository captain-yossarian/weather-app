use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::{ParseError, Url};

use std::{collections::HashMap, error::Error};

const API_KEY: &str = "QUp0o9h0Ir30CfhAGsolH5wUDHRUKAbZ";
const BASE_URL: &str = "http://dataservice.accuweather.com";
const REGIONS: &str = "locations/v1/regions";
const URL_LOCATIONS: &str = "locations/v1/adminareas/ua";
const CURRENT_CONDITIONS: &str = "hcurrentconditions/v1";
const ONE_DAY: &str = "forecasts/v1/daily/1day/61";
const LANGUAGES: &str = "translations/v1/languages";
const CITY_SEARCH: &str = "locations/v1/cities/search";

const APIKEY_TUPLE: (&str, &str) = ("apikey", API_KEY);
struct URLBuilder {
    url: String,
}

impl URLBuilder {
    pub fn new(host: String) -> Self {
        URLBuilder { url: host }
    }
}
fn url(route: &str) -> String {
    // format!("{}/{}?apikey={API_KEY}", BASE_URL, route)
    format!("{}/{}", BASE_URL, route)
}

fn to_json(response: String) -> Value {
    match serde_json::from_str(&response) {
        Ok(value) => value,
        Err(_) => serde_json::Value::String("".to_string()),
    }
}

async fn make_request(url: &str) -> Result<String, Box<dyn Error>> {
    let result = reqwest::get(url).await?.text().await?;
    Ok(result)
}

pub struct AccuProvider {
    url_base: Url,
}

impl AccuProvider {
    pub async fn new() -> Self {
        let url_base = Url::parse("http://dataservice.accuweather.com").unwrap();
        AccuProvider { url_base }
    }


    pub async fn city_search(self, city: &str) -> Result<Value, Box<dyn Error>> {
        let path = &url(CITY_SEARCH);
        let base = Url::parse_with_params(path, &[("q", city), APIKEY_TUPLE]).unwrap();

        let resp = reqwest::get(base).await?.text().await?;
        Ok(to_json(resp))
    }
}
