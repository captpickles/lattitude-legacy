use crate::accuweather::daily_forecast::DailyForecast;
use crate::accuweather::hourly_forecast::HourlyForecast;
use crate::calendar::Event;
use crate::netatmo::{Humidity, Pressure, Rain, Temperature, Wind};
use crate::purple::Aqi;

#[derive(PartialEq)]
pub struct DisplayData {
    //pub time: DateTime<Utc>,
    pub now: Option<NowData>,
    pub daily_forecast: Option<Vec<DailyForecast>>,
    pub hourly_forecast: Option<Vec<HourlyForecast>>,
    pub events: Option<Vec<Event>>,
    pub birds: Option<Vec<String>>,
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
