mod models;

use models::weather_api;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    let city = "Amersfoort";
    let n_days = 1;

    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={city}&days={n_days}&aqi=no&alerts=no"
    );

    let body = reqwest::get(url).await?.text().await?;
    let response: weather_api::Response = serde_json::from_str(&body)?;
    let location = response.location.name;
    let current_temp = response.current.temp_c;
    let condition = response.current.condition.text;

    println!("Current weather in {location}: {current_temp} degrees Celsius ({condition})");
    Ok(())
}
