use crate::errors::WeatherError;
use reqwest::IntoUrl;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::error::Error as StdError;
use url::Url;

const API_KEY: &str = "QUp0o9h0Ir30CfhAGsolH5wUDHRUKAbZ";
const BASE_URL: &str = "http://dataservice.accuweather.com";
const REGIONS: &str = "locations/v1/regions";
const URL_LOCATIONS: &str = "locations/v1/adminareas/ua";
const CURRENT_CONDITIONS: &str = "hcurrentconditions/v1";
const ONE_DAY: &str = "forecasts/v1/daily/1day/61";
const LANGUAGES: &str = "translations/v1/languages";
const CITY_SEARCH: &str = "locations/v1/cities/search";

const APIKEY_TUPLE: (&str, &str) = ("apikey", API_KEY);

fn with_path(route: &str) -> String {
    format!("{}/{}", BASE_URL, route)
}

fn to_json(response: String) -> Value {
    match serde_json::from_str(&response) {
        Ok(value) => value,
        Err(_) => serde_json::Value::String("".to_string()),
    }
}
fn to_struct<T>(json: Value) -> Result<T, WeatherError>
where
    T: DeserializeOwned,
{
    let result = serde_json::from_value::<T>(json);
    match result {
        Ok(value) => Ok(value),
        Err(err) => Err(WeatherError::SerdeError(err)),
    }
}

async fn make_request<T: IntoUrl>(url: T) -> Result<String, WeatherError> {
    let result = reqwest::get(url).await?.text().await?;
    Ok(result)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AdministrativeArea {
    CountryID: String,
    EnglishName: String,
    EnglishType: String,
    ID: String,
    Level: i32,
    LocalizedName: String,
    LocalizedType: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CitySearch {
    AdministrativeArea: AdministrativeArea,
}

pub struct AccuProvider {}

impl AccuProvider {
    pub async fn new() -> Self {
        AccuProvider {}
    }

    pub async fn request_city_search(self, city: &str) -> Result<String, WeatherError> {
        let base = Url::parse_with_params(
            &format!("{}/{}", BASE_URL, CITY_SEARCH),
            &[("q", city), APIKEY_TUPLE],
        )?;
        make_request(base).await
    }

    pub async fn city_search(self, city: &str) -> Result<Vec<CitySearch>, WeatherError> {
        let url = self.request_city_search(city).await?;
        let city_search = to_struct::<Vec<CitySearch>>(to_json(url));

        city_search
    }

    pub async fn location(self) -> Result<Vec<CitySearch>, WeatherError> {
        let path = with_path(URL_LOCATIONS);
        println!("{}", path);
        let base = Url::parse_with_params(&path, &[APIKEY_TUPLE]).unwrap();

        let resp = reqwest::get(base).await?.text().await?;
        to_struct::<Vec<CitySearch>>(to_json(resp))
    }
}
