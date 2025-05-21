mod api;
mod cli;
mod config;
mod models;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::get_config().expect("Could not determine configuration");

    if let Ok(x) = std::env::var("WEATHER_API_KEY") {
        config.api_key = Some(x);
    }
    let api_key = config.api_key.expect(
        "No API was provided as an environment variable or through the configuration file!",
    );

    let cli = cli::Cli::parse();

    let city = cli
        .city
        .or(config.city)
        .ok_or("No city was provided as an argument or through the configuration file!")?;
    let n_days = cli.forecast.or(config.n_forecast_days).or(Some(1)).unwrap();
    let n_hours = cli.hours.or(config.n_forecast_hours);

    let response = api::request_weather_data(api_key, city, &n_days)?;
    print_weather(response, n_days, n_hours);

    Ok(())
}

fn print_weather(response: models::weather_api::Response, n_days: i32, n_hours: Option<i32>) {
    println!("{:?}", response)
}
