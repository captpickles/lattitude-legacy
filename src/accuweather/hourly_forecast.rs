use chrono::{DateTime, Local};
use serde::Deserialize;
use crate::accuweather::daily_forecast::Temperature;

#[derive(Debug, Clone, Deserialize)]
pub struct Envelope(pub Vec<HourlyForecast>);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HourlyForecast {
    pub date_time: DateTime<Local>,
    pub has_precipitation: bool,
    pub temperature: HourlyTemperature,
    pub precipitation_probability: u8,
    pub weather_icon: u8,
    pub icon_phrase: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HourlyTemperature {
    pub value: f32,
}