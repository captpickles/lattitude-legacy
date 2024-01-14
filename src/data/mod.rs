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
use crate::birdnet::BirdNetClient;
use crate::state::{state, State};

#[allow(clippy::module_inception)]
pub mod data;

pub struct CachedData<T> {
    data: RefCell<Option<T>>,
    as_of: RefCell<Option<DateTime<Utc>>>,
    fetch: Box<dyn Fn(&State) -> Pin<Box<dyn Future<Output = Result<T, anyhow::Error>>>>>,
    cadence: Box<dyn Fn() -> Duration>,
}

impl<T: Clone> CachedData<T> {
    pub async fn get(&self, state: &State) -> Result<Option<T>, anyhow::Error> {
        if self.needs_fetch() {
            let data = (self.fetch)(&state).await.map(|inner| Some(inner))?;
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
    birdnet: CachedData<Vec<String>>,
}

fn birdnet_cadence() -> Duration {
    Duration::minutes(10)
}

fn fetch_birdnet(state: &State) -> Pin<Box<dyn Future<Output = Result<Vec<String>, anyhow::Error>>>> {
    let state = state.clone();
    Box::pin(async move {
        println!("fetch birdnet");
        let client = BirdNetClient::new();
        let birds = client.recent_detections(&state).await?;
        Ok(birds)
    })
}

fn calendar_cadence() -> Duration {
    Duration::days(1)
}

fn fetch_calendar(state: &State) -> Pin<Box<dyn Future<Output = Result<Vec<Event>, anyhow::Error>>>> {
    let state = state.clone();
    Box::pin(async move {
        println!("fetch calendar");
        let client = calendar::CalendarClient::new();
        let events = client.events(&state).await?;
        Ok(events)
    })
}

fn netatmo_cadence() -> Duration {
    Duration::minutes(15)
}

fn fetch_netatmo(state: &State) -> Pin<Box<dyn Future<Output = Result<NetatmoData, anyhow::Error>>>> {
    let state = state.clone();
    Box::pin(async move {
        println!("fetch netatmo");
        let netatmo_client = netatmo::get_client(&state).await?;
        let netatmo_data = netatmo_client.get_station_data().await?;
        Ok(netatmo_data)
    })
}

fn purple_cadence() -> Duration {
    Duration::hours(2)
}

fn fetch_purple(state: &State) -> Pin<Box<dyn Future<Output = Result<Aqi, anyhow::Error>>>> {
    let state = state.clone();
    Box::pin(async move {
        println!("fetch purple");
        let purple_client = purple::PurpleClient::new();
        let aqi = purple_client.get_aqi(&state).await?;
        Ok(aqi)
    })
}

pub fn accuweather_cadence() -> Duration {
    Duration::minutes(30)
}

fn fetch_accuweather_daily_forecast(state: &State
) -> Pin<Box<dyn Future<Output = Result<Vec<DailyForecast>, anyhow::Error>>>> {
    let state = state.clone();
    Box::pin(async move {
        println!("fetch accuweather");
        let client = accuweather::AccuWeatherClient::new();
        let forecast = client.daily_forecast(&state).await?;
        Ok(forecast)
    })
}

fn fetch_accuweather_hourly_forecast(state: &State
) -> Pin<Box<dyn Future<Output = Result<Vec<HourlyForecast>, anyhow::Error>>>> {
    let state = state.clone();
    Box::pin(async move {
        println!("fetch accuweather hourly");
        let client = accuweather::AccuWeatherClient::new();
        let forecast = client.hourly_forecasts(&state).await?;
        Ok(forecast)
    })
}

impl DataSource {
    pub fn new() -> Self {
        Self {
            birdnet: CachedData {
                data: RefCell::new(None),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_birdnet),
                cadence: Box::new(birdnet_cadence),
            },
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

    pub async fn get(&self, state: &State) -> Result<DisplayData, anyhow::Error> {
        Ok(DisplayData {
            //time: Utc::now(),
            now: self.get_now(&state).await?,
            daily_forecast: self.get_daily_forecast(&state).await?,
            hourly_forecast: self.get_hourly_forecast(&state).await?,
            events: self.calendar.get(&state).await?.unwrap_or(vec![]),
            birds: self.birdnet.get(&state).await?.unwrap_or(vec![]),
        })
    }

    async fn get_daily_forecast(&self, state: &State) -> Result<Vec<DailyForecast>, anyhow::Error> {
        Ok(self.accuweather_daily.get(&state).await?.unwrap_or(vec![]))
    }

    async fn get_hourly_forecast(&self, state: &State) -> Result<Vec<HourlyForecast>, anyhow::Error> {
        Ok(self.accuweather_hourly.get(&state).await?.unwrap_or(vec![]))
    }

    async fn get_now(&self, state: &State) -> Result<NowData, anyhow::Error> {
        let mut now_data = NowData {
            temp: None,
            humidity: None,
            pressure: None,
            wind: None,
            rain: None,
            aqi: None,
        };

        if let Some(netatmo) = self.netatmo.get(&state).await? {
            now_data.temp = netatmo.outside_temp();
            now_data.wind = netatmo.wind();
            now_data.rain = netatmo.rain();
            now_data.humidity = netatmo.humidity();
            now_data.pressure = netatmo.pressure();
        }

        if let Ok(purple) = self.purple.get(&state).await {
            now_data.aqi = purple
        }

        Ok(now_data)
    }
}
