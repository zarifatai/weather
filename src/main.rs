pub mod cli;
mod models;

use std::io::BufReader;
use std::path::PathBuf;
use std::{fs, io::Write};

use serde::{Deserialize, Serialize};

use models::weather_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = _get_config().expect("Could not determine configuration");

    if let Ok(x) = std::env::var("WEATHER_API_KEY") {
        config.api_key = Some(x);
    }

    let city = "Amersfoort";
    let n_days = 1;
    let api_key = config.api_key.unwrap();

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

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    city: Option<String>,
    n_forecast_days: Option<i32>,
    api_key: Option<String>,
}

fn _get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = _determine_config_path()
        .expect("Could not determine config file path! Please manually create settings file in ~/.config/weather/config.json");

    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;

    Ok(config)
}

fn _determine_config_path() -> std::io::Result<PathBuf> {
    if let Ok(x) = std::env::var("WEATHER_CONFIG_FILE_PATH") {
        Ok(PathBuf::from(x))
    } else {
        _create_config_file()
    }
}

fn _create_config_file() -> std::io::Result<PathBuf> {
    let home_dir = dirs::home_dir().expect("Failed to determine home directory");
    let config_dir = home_dir.join(".config").join("weather");
    let config_path = config_dir.join("settings.json");

    // Create the parent directory if it doesn't exist
    fs::create_dir_all(&config_dir)?;

    // Create the file if it doesn't exist
    if !config_path.exists() {
        let mut file = fs::File::create(&config_path)?;
        file.write_all(b"{}")?;
    }
    Ok(config_path)
}
