use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub location: Location,
    pub current: CurrentConditions,
    pub forecast: Forecast,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub localtime: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentConditions {
    pub last_updated_epoch: i32,
    pub last_updated: String,
    pub temp_c: f32,
    pub condition: ConditionText,
    pub wind_kph: f32,
    pub feelslike_c: f32,
    pub uv: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayConditions {
    pub maxtemp_c: f32,
    pub mintemp_c: f32,
    pub condition: ConditionText,
    pub maxwind_kph: f32,
    pub daily_chance_of_rain: f32,
    pub uv: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HourConditions {
    pub time_epoch: i32,
    pub temp_c: f32,
    pub condition: ConditionText,
    pub wind_kph: f32,
    pub chance_of_rain: f32,
    pub feelslike_c: f32,
    pub uv: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConditionText {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDay {
    pub date_epoch: i32,
    pub day: DayConditions,
    pub hour: Vec<HourConditions>,
}

pub trait CommonConditionsAttributes {
    fn feelslike_c(&self) -> &f32;
    fn condition(&self) -> &ConditionText;
    fn temp_c(&self) -> &f32;
    fn uv(&self) -> &f32;
    fn wind_kph(&self) -> &f32;
}

macro_rules! impl_common_conditions_attributes {
    ($t:ty) => {
        impl CommonConditionsAttributes for $t {
            fn feelslike_c(&self) -> &f32 {
                &self.feelslike_c
            }
            fn condition(&self) -> &ConditionText {
                &self.condition
            }
            fn temp_c(&self) -> &f32 {
                &self.temp_c
            }
            fn uv(&self) -> &f32 {
                &self.uv
            }
            fn wind_kph(&self) -> &f32 {
                &self.wind_kph
            }
        }
    };
}

impl_common_conditions_attributes!(CurrentConditions);
impl_common_conditions_attributes!(HourConditions);
