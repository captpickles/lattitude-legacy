use serde::Deserialize;
use serde_json::{Map, Value};
use serde_json::value::RawValue;

#[derive(Deserialize, Debug)]
pub struct Envelope {
    pub body: Body,
}

#[derive(Deserialize, Debug)]
pub struct Body {
    pub devices: Vec<Device>
}

#[derive(Deserialize, Debug)]
pub struct Device {
    pub _id: String,
    pub station_name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub data_type: Vec<String>,
    pub dashboard_data: Value,
    pub modules: Vec<Map<String,Value>>,
}
