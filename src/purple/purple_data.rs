use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Envelope {
    pub sensor: Sensor,
}

#[derive(Deserialize, Debug)]
pub struct Sensor {
    #[serde(rename = "pm2.5")]
    pub pm2_5: f64,
    pub stats: Stats,
}

#[derive(Deserialize, Debug)]
pub struct Stats {
    #[serde(rename = "pm2.5_60minute")]
    pub pm2_5_1hour: f64,
    #[serde(rename = "pm2.5_6hour")]
    pub pm2_5_6hour: f64,
    #[serde(rename = "pm2.5_24hour")]
    pub pm2_5_24hour: f64,
}
