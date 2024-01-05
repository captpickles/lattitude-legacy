use crate::accuweather::daily_forecast::DailyForecast;
use crate::accuweather::hourly_forecast::HourlyForecast;
use crate::calendar::Event;
use crate::data::data::{DisplayData, NowData};
use crate::netatmo::{NetatmoData, };
use crate::purple::Aqi;
use crate::{accuweather, calendar, netatmo, purple};
use chrono::{DateTime, Duration, Utc};
use std::cell::{RefCell};
use std::future::Future;
use std::pin::Pin;

#[allow(clippy::module_inception)]
pub mod data;

pub struct CachedData<T> {
    data: RefCell<Option<T>>,
    as_of: RefCell<Option<DateTime<Utc>>>,
    fetch: Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<T, anyhow::Error>>>>>,
    cadence: Box<dyn Fn() -> Duration>,
}

impl<T: Clone> CachedData<T> {
    pub async fn get(&self) -> Result<Option<T>, anyhow::Error> {
        if self.needs_fetch() {
            let data = (self.fetch)().await.map(|inner| Some(inner))?;
            self.as_of.borrow_mut().replace(Utc::now());
            *self.data.borrow_mut() = data;
        }

        Ok(self.data.borrow().clone())
    }

    pub fn needs_fetch(&self) -> bool {
        if let Some(as_of) = &*self.as_of.borrow() {
            let age = Utc::now() - as_of;
            if age > (self.cadence)() {
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}

pub struct DataSource {
    calendar: CachedData<Vec<Event>>,
    netatmo: CachedData<NetatmoData>,
    purple: CachedData<Aqi>,
    accuweather_daily: CachedData<Vec<DailyForecast>>,
    accuweather_hourly: CachedData<Vec<HourlyForecast>>,
}

fn calendar_cadence() -> Duration {
    Duration::days(1)
}

fn fetch_calendar() -> Pin<Box<dyn Future<Output = Result<Vec<Event>, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch calendar");
        let client = calendar::CalendarClient::new();
        let events = client.events().await?;
        Ok(events)
    })
}

fn netatmo_cadence() -> Duration {
    Duration::minutes(15)
}

fn fetch_netatmo() -> Pin<Box<dyn Future<Output = Result<NetatmoData, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch netatmo");
        let netatmo_client = netatmo::get_client().await?;
        let netatmo_data = netatmo_client.get_station_data().await?;
        Ok(netatmo_data)
    })
}

fn purple_cadence() -> Duration {
    Duration::hours(2)
}

fn fetch_purple() -> Pin<Box<dyn Future<Output = Result<Aqi, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch purple");
        let purple_client = purple::PurpleClient::new();
        let aqi = purple_client.get_aqi().await?;
        Ok(aqi)
    })
}

pub fn accuweather_cadence() -> Duration {
    Duration::minutes(30)
}

fn fetch_accuweather_daily_forecast(
) -> Pin<Box<dyn Future<Output = Result<Vec<DailyForecast>, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch accuweather");
        let client = accuweather::AccuWeatherClient::new();
        let forecast = client.daily_forecast().await?;
        Ok(forecast)
    })
}

fn fetch_accuweather_hourly_forecast(
) -> Pin<Box<dyn Future<Output = Result<Vec<HourlyForecast>, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch accuweather hourly");
        let client = accuweather::AccuWeatherClient::new();
        let forecast = client.hourly_forecasts().await?;
        Ok(forecast)
    })
}

impl DataSource {
    pub fn new() -> Self {
        Self {
            calendar: CachedData {
                data: RefCell::new(None),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_calendar),
                cadence: Box::new(calendar_cadence),
            },
            netatmo: CachedData {
                data: RefCell::new(None),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_netatmo),
                cadence: Box::new(netatmo_cadence),
            },
            purple: CachedData {
                data: RefCell::new(None),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_purple),
                cadence: Box::new(purple_cadence),
            },
            accuweather_daily: CachedData {
                data: RefCell::new(None),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_accuweather_daily_forecast),
                cadence: Box::new(accuweather_cadence),
            },
            accuweather_hourly: CachedData {
                data: RefCell::new(None),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_accuweather_hourly_forecast),
                cadence: Box::new(accuweather_cadence),
            },
        }
    }

    pub async fn get(&self) -> Result<DisplayData, anyhow::Error> {
        Ok(DisplayData {
            //time: Utc::now(),
            now: self.get_now().await?,
            daily_forecast: self.get_daily_forecast().await?,
            hourly_forecast: self.get_hourly_forecast().await?,
            events: self.calendar.get().await?.unwrap_or(vec![]),
        })
    }

    async fn get_daily_forecast(&self) -> Result<Vec<DailyForecast>, anyhow::Error> {
        if let Some(forecast) = self.accuweather_daily.get().await? {
            Ok(forecast)
        } else {
            Ok(vec![])
        }
    }

    async fn get_hourly_forecast(&self) -> Result<Vec<HourlyForecast>, anyhow::Error> {
        if let Some(forecast) = self.accuweather_hourly.get().await? {
            Ok(forecast)
        } else {
            Ok(vec![])
        }
    }

    async fn get_now(&self) -> Result<NowData, anyhow::Error> {
        let mut now_data = NowData {
            temp: None,
            humidity: None,
            pressure: None,
            wind: None,
            rain: None,
            aqi: None,
        };

        if let Some(netatmo) = self.netatmo.get().await? {
            now_data.temp = netatmo.outside_temp();
            now_data.wind = netatmo.wind();
            now_data.rain = netatmo.rain();
            now_data.humidity = netatmo.humidity();
            now_data.pressure = netatmo.pressure();
        }

        if let Ok(purple) = self.purple.get().await {
            now_data.aqi = purple
        }

        Ok(now_data)
    }
}
