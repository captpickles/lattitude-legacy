use clap::Parser;
use crate::accuweather::daily_forecast::Snow;
use crate::purple::purple_data::Envelope;
use crate::state::state;

mod purple_data;

const GET_SENSOR_DATA_URL: &str = "https://api.purpleair.com/v1/sensors";

pub struct PurpleClient {}

impl PurpleClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_aqi(&self) -> Result<Aqi, anyhow::Error> {
        let state = state().purple;

        let url = format!("{}/{}", GET_SENSOR_DATA_URL, state.sensor_index);
        let response = reqwest::Client::new()
            .get(url)
            .query(&[("fields", "pm2.5,pm2.5_60minute,pm2.5_6hour,pm2.5_24hour")])
            .header("X-API-KEY", state.api_key)
            .send()
            .await?;

        let data: Envelope = response.json().await?;

        Ok(Aqi {
            current: data.sensor.pm2_5,
            one_hour: data.sensor.stats.pm2_5_1hour,
            six_hour: data.sensor.stats.pm2_5_6hour,
            twenty_four_hour: data.sensor.stats.pm2_5_24hour,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Aqi {
    pub current: f64,
    pub one_hour: f64,
    pub six_hour: f64,
    pub twenty_four_hour: f64,
}