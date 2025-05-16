use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    let city = "Amersfoort";
    let n_days = 1;

    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={city}&days={n_days}&aqi=no&alerts=no"
    );

    let body = reqwest::get(url).await?.text().await?;

    println!("{body:?}");
    Ok(())
}
