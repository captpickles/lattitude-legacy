use crate::accuweather::daily_forecast::DailyForecast;
use crate::accuweather::hourly_forecast::HourlyForecast;
use crate::calendar::Event;
use crate::netatmo::{Humidity, Pressure, Rain, Temperature, Wind};
use crate::purple::Aqi;
use chrono::{DateTime, Utc};

#[derive(PartialEq)]
pub struct DisplayData {
    //pub time: DateTime<Utc>,
    pub now: NowData,
    pub daily_forecast: Vec<DailyForecast>,
    pub hourly_forecast: Vec<HourlyForecast>,
    pub events: Vec<Event>,
    pub birds: Vec<String>,
}

#[derive(PartialEq)]
pub struct NowData {
    pub temp: Option<Temperature>,
    pub humidity: Option<Humidity>,
    pub pressure: Option<Pressure>,
    pub wind: Option<Wind>,
    pub rain: Option<Rain>,
    pub aqi: Option<Aqi>,
}
