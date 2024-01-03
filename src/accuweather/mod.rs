mod location;
pub mod daily_forecast;

pub mod hourly_forecast;

use std::cell::{Cell, Ref, RefCell};
use crate::accuweather::daily_forecast::DailyForecast;
use crate::accuweather::hourly_forecast::HourlyForecast;
use crate::accuweather::location::Location;
use crate::state::state;

const GEOPOSITION_SEARCH_URL: &str = "http://dataservice.accuweather.com/locations/v1/cities/geoposition/search";
const DAILY_FORECAST_URL: &str = "http://dataservice.accuweather.com/forecasts/v1/daily/5day";
const HOURLY_FORECAST_URL: &str = "http://dataservice.accuweather.com/forecasts/v1/hourly/12hour";

pub struct AccuWeatherClient {
    location_key: RefCell<Option<String>>,
}

impl AccuWeatherClient {

    pub fn new() -> Self {
        Self {
            location_key: RefCell::new(None),
        }
    }

    pub async fn get_location_key(&self) -> Result<String, anyhow::Error>{
        let state = state();

        if let Some(location_key) = &*self.location_key.borrow() {
            return Ok(location_key.clone());
        }

        let location: Location = reqwest::Client::new()
            .get(GEOPOSITION_SEARCH_URL)
            .query(
                &[
                    ("apikey", state.accuweather.api_key),
                    ("q", format!("{},{}", state.location.lat, state.location.lon)),
                ]
            )
            .send()
            .await?
            .json()
            .await?;

        self.location_key.borrow_mut().replace(location.key.clone());

        Ok(location.key)
    }

    pub async fn daily_forecast(&self) -> Result<Vec<DailyForecast>, anyhow::Error> {
        let state = state();
        let location_key = self.get_location_key().await?;

        let url = format!("{}/{}", DAILY_FORECAST_URL, location_key);

        let forecast: daily_forecast::Envelope = reqwest::Client::new()
        //let forecast: Value= reqwest::Client::new()
            .get(url)
            .query(
                &[
                    ("apikey", state.accuweather.api_key),
                    ("details", "true".to_string()),
                ]
            )
            .send()
            .await?
            .json()
            .await?;

        Ok( forecast.daily_forecasts)
    }

    pub async fn hourly_forecasts(&self) -> Result<Vec<HourlyForecast>, anyhow::Error> {
        let state = state();
        let location_key = self.get_location_key().await?;

        let url = format!("{}/{}", HOURLY_FORECAST_URL, location_key);

        let forecast: hourly_forecast::Envelope = reqwest::Client::new()
            .get(url)
            .query(
                &[
                    ("apikey", state.accuweather.api_key),
                    ("details", "true".to_string()),
                ]
            )
            .send()
            .await?
            .json()
            .await?;

        Ok( forecast.0)

    }

}