mod detections;

use serde_json::Value;
use crate::birdnet::detections::Envelope;
use crate::state::{state, State};

const BASE_URL: &str = "https://app.birdweather.com/api/v1/stations";

#[derive(Default)]
pub struct BirdNetClient {

}

impl BirdNetClient {
    pub fn new() -> Self {
        Self {

        }
    }

    pub async fn recent_detections(&self, state: &State) -> Result<Vec<String>, anyhow::Error> {
        let data: Envelope = reqwest::Client::new()
            .get(format!("{}/{}/detections", BASE_URL, &state.birdnet.token) )
            /*
            .query(&[
                (
                    "limit", "100"
                ),
            ])

             */
            .send()
            .await?
            .json()
            .await?;

        let mut detections = Vec::new();

        for each in data.detections {
            if ! detections.contains(&each.species.common_name) {
                detections.push( each.species.common_name.clone());
            }

            if detections.len() >= 10 {
                break;
            }
        }

        println!("{:#?}", detections);

        Ok(detections)
    }

}