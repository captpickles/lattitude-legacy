use std::hash::{Hash, Hasher};
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Envelope {
    pub daily_forecasts: Vec<DailyForecast>,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DailyForecast {
    pub date: DateTime<Local>,
    pub sun: Sun,
    pub moon: Moon,
    pub temperature: Temperature,
    pub day: Details,
    pub night: Details,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Sun {
    pub rise: DateTime<Local>,
    pub set: DateTime<Local>,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Moon {
    pub rise: Option<DateTime<Local>>,
    pub set: Option<DateTime<Local>>,
    pub phase: String,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Temperature {
    pub minimum: TempValue,
    pub maximum: TempValue,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TempValue {
    pub value: f32,
}

impl Hash for TempValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write( &self.value.to_be_bytes())
    }
}


#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Details {
    pub icon: u8,
    pub icon_phrase: String,
    pub short_phrase: String,
    pub long_phrase: String,
    pub precipitation_probability: u8,
    pub total_liquid: TotalLiquid,
    pub snow: Snow,
    pub rain: Rain,
    pub ice: Ice,
    pub wind: Wind,
    pub wind_gust: Wind,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Wind {
    pub speed: WindSpeed,
    pub direction: WindDirection,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WindSpeed {
    pub value: f32,
}

impl Hash for WindSpeed {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write( &self.value.to_be_bytes())
    }
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WindDirection {
    pub degrees: u16,
    pub localized: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TotalLiquid {
    pub value: f32,
}

impl Hash for TotalLiquid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write( &self.value.to_be_bytes())
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Snow {
    pub value: f32,
}

impl Hash for Snow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write( &self.value.to_be_bytes())
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Rain {
    pub value: f32,
}

impl Hash for Rain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write( &self.value.to_be_bytes())
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Ice {
    pub value: f32,
}

impl Hash for Ice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write( &self.value.to_be_bytes())
    }
}
