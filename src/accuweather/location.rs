use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "Key")]
    pub key: String,
}
