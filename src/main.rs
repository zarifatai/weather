mod api;
mod cli;
mod config;
mod models;

use std::io::{self, Write};

use chrono::{NaiveDateTime, Timelike};
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
        .days
        .or(config.n_forecast_days)
        .unwrap_or(DEFAULT_N_DAYS);
    let n_hours = cli
        .hours
        .or(config.n_forecast_hours)
        .unwrap_or(DEFAULT_N_HOURS);

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
    print_today(&response);
    print_divider();
    print_coming_hours(&response, n_hours);
    print_divider();
    print_coming_days(&response, n_days);
    print_divider();
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

fn print_today(response: &models::weather_api::Response) {
    let forecast_today = &response.forecast.forecastday[0];
    let day_conditions = &forecast_today.day;
    let max_temp_c = day_conditions.maxtemp_c;
    let min_temp_c = day_conditions.mintemp_c;
    let max_wind_kph = day_conditions.maxwind_kph;
    println!(
        "Min/max temperature: {min_temp_c}/{max_temp_c}󰔄 | Max wind speed: {max_wind_kph} km/h"
    )
}

fn print_coming_hours(response: &models::weather_api::Response, n_hours: i32) {
    println!(
        "Coming {n_hours} hour{}:\n",
        if n_hours == 1 { "" } else { "s" }
    );
    let forecast_today = &response.forecast.forecastday[0];
    let current_datetime = &response.current.last_updated;
    let coming_hours = get_coming_hours(current_datetime, &n_hours);
    for hour in coming_hours {
        println!("{hour}:00:");
        print_conditions(&forecast_today.hour[hour]);
    }
}

fn print_coming_days(response: &models::weather_api::Response, n_days: i32) {
    println!(
        "Coming {n_days} day{}:\n",
        if n_days == 1 { "" } else { "s" }
    );
    let forecast_days = &response.forecast.forecastday;

    // The first element is skipped because it's the forecast for today
    let n_days = n_days as usize;
    for day in forecast_days.iter().take(n_days + 1).skip(1) {
        let max_temp_c = day.day.maxtemp_c;
        let min_temp_c = day.day.mintemp_c;
        let condition = &day.day.condition.text;
        let max_wind_kph = day.day.maxwind_kph;
        let chance_of_rain = day.day.daily_chance_of_rain;
        let uv = day.day.uv;
        let day = &day.date;
        println!("{day}:");
        println!(
            "Min/max temperature: {min_temp_c}/{max_temp_c}󰔄  ({condition})| Max wind speed: {max_wind_kph} km/h | Chance of rain: {chance_of_rain}% | UV index: {uv}"
        );
    }
}

fn print_footer(response: &models::weather_api::Response) {
    let last_updated = &response.current.last_updated;
    println!("Last updated: {last_updated} (local time)");
}

fn print_divider() {
    let char_divider = '-'.to_string();
    println!("{}", char_divider.repeat(59));
}

fn get_coming_hours(current_datetime: &str, n_hours: &i32) -> Vec<usize> {
    let current_datetime =
        NaiveDateTime::parse_from_str(current_datetime, "%Y-%m-%d %H:%M").unwrap();

    let mut coming_hours = Vec::new();
    let current_hour = current_datetime.hour() as usize;
    for i in 0..*n_hours {
        coming_hours.push((current_hour + 1 + i as usize) % 24);
    }
    coming_hours
}

fn set_weather_api_key(config: &mut Config) {
    if let Ok(x) = std::env::var("WEATHER_API_KEY") {
        config.api_key = Some(x);
    }
}
