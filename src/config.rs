use std::io::BufReader;
use std::path::PathBuf;
use std::{fs, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub city: Option<String>,
    pub n_forecast_days: Option<i32>,
    pub n_forecast_hours: Option<i32>,
    pub api_key: Option<String>,
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = determine_config_path()
        .expect("Could not determine config file path! Please manually create settings file in ~/.config/weather/config.json");

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;

    Ok(config)
}

fn determine_config_path() -> std::io::Result<PathBuf> {
    if let Ok(x) = std::env::var("WEATHER_CONFIG_FILE_PATH") {
        Ok(PathBuf::from(x))
    } else {
        create_config_file()
    }
}

fn create_config_file() -> std::io::Result<PathBuf> {
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
