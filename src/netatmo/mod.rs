#![allow(unused)]

use crate::netatmo::station_data::Envelope;
use crate::state::{state, update_state};
use serde::Deserialize;
use serde_json::Value;
use crate::accuweather::daily_forecast::Snow;

mod station_data;

const AUTH_URL: &str = "https://api.netatmo.com/oauth2/authorize";
const TOKEN_URL: &str = "https://api.netatmo.com/oauth2/token";

const GET_STATIONS_DATA: &str = "https://api.netatmo.com/api/getstationsdata";

#[derive(Deserialize, Debug)]
pub struct RefreshedToken {
    access_token: String,
    refresh_token: String,
}

pub async fn get_client() -> Result<NetatmoClient, anyhow::Error> {
    let state = state().netatmo;

    let client = reqwest::Client::new();
    let result = client
        .post(TOKEN_URL)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", &state.refresh_token),
            ("client_id", &state.client_id),
            ("client_secret", &state.client_secret),
        ])
        .send()
        .await?;

    let refreshed: RefreshedToken = result.json().await?;

    if state.refresh_token != refreshed.refresh_token {
        update_state(|update| update.netatmo.refresh_token = refreshed.refresh_token);
    }

    Ok(NetatmoClient {
        access_token: refreshed.access_token,
    })
}

#[derive(Debug, Default, Clone)]
pub struct NetatmoData {
    pub inside: Vec<WeatherData>,
    pub outside: Vec<WeatherData>,
}

impl NetatmoData {
    pub fn outside_temp(&self) -> Option<Temperature> {
        self.outside
            .iter()
            .find_map(|e| {
                if let WeatherData::Temperature(temp) = e {
                    Some(temp)
                } else {
                    None
                }
            })
            .cloned()
    }

    pub fn wind(&self) -> Option<Wind> {
        self.outside
            .iter()
            .find_map(|e| {
                if let WeatherData::Wind(wind) = e {
                    Some(wind)
                } else {
                    None
                }
            })
            .cloned()
    }

    pub fn rain(&self) -> Option<Rain> {
        self.outside
            .iter()
            .find_map(|e| {
                if let WeatherData::Rain(rain) = e {
                    Some(rain)
                } else {
                    None
                }
            })
            .cloned()
    }

    pub fn humidity(&self) -> Option<Humidity> {
        self.outside
            .iter()
            .find_map(|e| {
                if let WeatherData::Humidity(humidity) = e {
                    Some(humidity)
                } else {
                    None
                }
            })
            .cloned()
    }

    pub fn pressure(&self) -> Option<Pressure> {
        self.outside
            .iter()
            .find_map(|e| {
                if let WeatherData::Pressure(pressure) = e {
                    Some(pressure)
                } else {
                    None
                }
            })
            .cloned()
    }
}

#[derive(Debug)]
pub struct NetatmoClient {
    access_token: String,
}

impl NetatmoClient {
    pub async fn get_station_data(&self) -> Result<NetatmoData, anyhow::Error> {
        let response = reqwest::Client::new()
            .post(GET_STATIONS_DATA)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        let data: Envelope = response.json().await?;

        let mut netatmo_data = NetatmoData::default();

        for device in data.body.devices {
            for data_type in &device.data_type {
                let dashboard_data = &device.dashboard_data;
                if let Some(result) = convert(data_type, dashboard_data) {
                    netatmo_data.inside.push(result);
                }
            }
            for module in device.modules {
                if let Some(Value::Array(data_types)) = module.get("data_type") {
                    for data_type in data_types {
                        if data_type.is_string() {
                            let data_type = data_type.as_str().unwrap();
                            if let Some(dashboard_data) = module.get("dashboard_data") {
                                if let Some(result) = convert(data_type, dashboard_data) {
                                    netatmo_data.outside.push(result);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(netatmo_data)
    }
}

fn convert(data_type: &str, dashboard_data: &Value) -> Option<WeatherData> {
    match data_type {
        "Wind" => {
            let wind: Wind = dashboard_data.into();
            Some(WeatherData::Wind(wind))
        }
        "Temperature" => {
            let temperature: Temperature = dashboard_data.into();
            Some(WeatherData::Temperature(temperature))
        }
        "Humidity" => {
            let humidity: Humidity = dashboard_data.into();
            Some(WeatherData::Humidity(humidity))
        }
        "Rain" => {
            let rain: Rain = dashboard_data.into();
            Some(WeatherData::Rain(rain))
        }
        "CO2" => {
            let co2: Co2 = dashboard_data.into();
            Some(WeatherData::Co2(co2))
        }
        "Noise" => {
            let noise: Noise = dashboard_data.into();
            Some(WeatherData::Noise(noise))
        }
        "Pressure" => {
            let pressure: Pressure = dashboard_data.into();
            Some(WeatherData::Pressure(pressure))
        }
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum WeatherData {
    Wind(Wind),
    Rain(Rain),
    Temperature(Temperature),
    Humidity(Humidity),
    Co2(Co2),
    Noise(Noise),
    Pressure(Pressure),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Wind {
    #[serde(rename = "max_wind_str")]
    pub max_wind_strength: u16,
    #[serde(rename = "WindStrength")]
    pub wind_strength: u16,
    #[serde(rename = "WindAngle")]
    pub wind_angle: u16,
    #[serde(rename = "GustStrength")]
    pub gust_strength: u16,
    #[serde(rename = "GustAngle")]
    pub gust_angle: u16,
}

impl From<&Value> for Wind {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Temperature {
    #[serde(rename = "Temperature")]
    pub temperature: Option<f32>,
    pub min_temp: Option<f32>,
    pub max_temp: Option<f32>,
    pub temp_trend: Option<Trend>,
}

impl From<&Value> for Temperature {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Humidity {
    #[serde(rename = "Humidity")]
    pub humidity: f32,
}

impl From<&Value> for Humidity {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

#[allow(unused)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Rain {
    #[serde(rename = "Rain")]
    rain: f32,
    sum_rain_1: f32,
    sum_rain_24: f32,
}

impl From<&Value> for Rain {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Noise {
    #[serde(rename = "Noise")]
    noise: u16,
}

impl From<&Value> for Noise {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Co2 {
    #[serde(rename = "CO2")]
    co2: u16,
}

impl From<&Value> for Co2 {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Pressure {
    #[serde(rename = "Pressure")]
    pressure: f32,
    #[serde(rename = "AbsolutePressure")]
    absolute_pressure: f32,
}

impl From<&Value> for Pressure {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Trend {
    Up,
    Down,
    Stable,
}
