use crate::models;

#[tokio::main]
pub async fn request_weather_data(
    api_key: String,
    city: String,
    n_days: &i32,
) -> Result<models::weather_api::Response, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={city}&days={n_days}&aqi=no&alerts=no"
    );

    let body = reqwest::get(url).await?.text().await?;
    let response: models::weather_api::Response = serde_json::from_str(&body)?;
    Ok(response)
}
