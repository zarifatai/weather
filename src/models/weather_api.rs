use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub location: Location,
    pub current: CurrentConditions,
    pub forecast: Forecast,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentConditions {
    pub last_updated_epoch: i32,
    pub temp_c: f32,
    pub condition: ConditionText,
    pub wind_kph: f32,
    pub feelslike_c: f32,
    pub uv: f32,
}

#[derive(Serialize, Deserialize)]
pub struct DayConditions {
    pub maxtemp_c: f32,
    pub mintemp_c: f32,
    pub condition: ConditionText,
    pub maxwind_kph: f32,
    pub daily_chance_of_rain: f32,
    pub uv: f32,
}

#[derive(Serialize, Deserialize)]
pub struct HourConditions {
    pub time_epoch: i32,
    pub temp_c: f32,
    pub condition: ConditionText,
    pub wind_kph: f32,
    pub chance_of_rain: f32,
    pub feelslike_c: f32,
    pub uv: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionText {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Forecast {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Serialize, Deserialize)]
pub struct ForecastDay {
    pub date_epoch: i32,
    pub day: DayConditions,
    pub hour: Vec<HourConditions>,
}
