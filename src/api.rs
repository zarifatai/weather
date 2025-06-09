use crate::models;

use reqwest::Client;
use std::time::Duration;

#[tokio::main]
pub async fn request_weather_data(
    api_key: String,
    city: String,
    n_days: &i32,
) -> Result<models::weather_api::Response, Box<dyn std::error::Error>> {
    // By n_days the current date is excluded so we increment it by 1
    let n_days = n_days + 1;
    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={city}&days={n_days}&aqi=no&alerts=no"
    );

    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
    let body = client.get(url).send().await?.text().await?;
    let response: models::weather_api::Response = serde_json::from_str(&body)?;
    Ok(response)
}
