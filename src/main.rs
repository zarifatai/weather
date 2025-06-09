mod api;
mod cli;
mod config;
mod models;

use std::io::{self, Write};

use chrono::{Duration, NaiveDateTime, Timelike};
use clap::Parser;
use crossterm::{
    ExecutableCommand, cursor,
    terminal::{Clear, ClearType},
};

use config::Config;

const DEFAULT_N_DAYS: i32 = 1;
const DEFAULT_N_HOURS: i32 = 2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::get_config().expect("Could not determine configuration");

    set_weather_api_key(&mut config);
    let api_key = config.api_key.expect(
        "No API was provided as an environment variable or through the configuration file!",
    );

    let cli = cli::Cli::parse();

    let city = cli
        .city
        .or(config.city)
        .ok_or("No city was provided as an argument or through the configuration file!")?;
    let n_days = cli
        .forecast
        .or(config.n_forecast_days)
        .or(Some(DEFAULT_N_DAYS))
        .unwrap();
    let n_hours = cli
        .hours
        .or(config.n_forecast_hours)
        .or(Some(DEFAULT_N_HOURS))
        .unwrap();

    let mut stdout = io::stdout();
    print!("Fetching weather data from the internet...");
    stdout.flush()?;
    let response = api::request_weather_data(api_key, city, &n_days)?;
    stdout
        .execute(cursor::MoveToColumn(0))?
        .execute(Clear(ClearType::CurrentLine))?;
    print_weather(response, n_days, n_hours);

    Ok(())
}

fn print_weather(response: models::weather_api::Response, n_days: i32, n_hours: i32) {
    print_location(&response);
    print_conditions(&response.current);
    print_divider();
    print_coming_hours(&response, &n_hours);
    print_divider();
    print_coming_days(&response, &n_days);
    print_footer(&response);
}

fn print_location(response: &models::weather_api::Response) {
    let city = &response.location.name;
    let region = &response.location.region;
    let country = &response.location.country;
    let local_time = &response.location.localtime;
    println!("{city}, {region}, {country} ({local_time})\n");
}

fn print_conditions<T: models::weather_api::CommonConditionsAttributes>(conditions: &T) {
    let feelslike_c = conditions.feelslike_c();
    let condition = conditions.condition();
    let temp_c = conditions.temp_c();
    let uv = conditions.uv();
    let wind_kph = conditions.wind_kph();

    println!("Feels like {feelslike_c}󰔄 ({})", condition.text);
    println!("Actual temperature: {temp_c}󰔄 | UV: {uv} | Wind speed: {wind_kph} km/h");
}

fn print_coming_hours(response: &models::weather_api::Response, n_hours: &i32) {
    println!(
        "Coming {n_hours} hour{}:",
        if *n_hours == 1 { "" } else { "s" }
    );
    let forecast_today = &response.forecast.forecastday[0];
    let current_datetime = &response.current.last_updated;
    let coming_hours = get_coming_hours(current_datetime, n_hours);
    for hour in coming_hours {
        println!("\n{hour}:00:");
        print_conditions(&forecast_today.hour[hour]);
    }
}

fn print_coming_days(response: &models::weather_api::Response, n_days: &i32) {
    todo!();
}

fn print_footer(response: &models::weather_api::Response) {
    let last_updated = &response.current.last_updated;
    println!("\nLast updated: {last_updated} (local time)");
}

fn print_divider() {
    let char_divider = '_'.to_string();
    println!("{}", char_divider.repeat(59));
}

fn get_coming_hours(current_datetime: &str, n_hours: &i32) -> Vec<usize> {
    let current_datetime =
        NaiveDateTime::parse_from_str(current_datetime, "%Y-%m-%d %H:%M").unwrap();

    let mut coming_hours = Vec::new();
    let current_hour = current_datetime.hour() as usize;
    for i in 0..*n_hours {
        coming_hours.push(current_hour + 1 + i as usize);
    }
    coming_hours
}

fn set_weather_api_key(config: &mut Config) {
    if let Ok(x) = std::env::var("WEATHER_API_KEY") {
        config.api_key = Some(x);
    }
}
