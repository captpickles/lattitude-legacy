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
use crate::state::{AccuWeatherState, BirdNetState, CalendarState, LocationState, NetatmoState, PurpleState, State};

#[allow(clippy::module_inception)]
pub mod data;

pub struct CachedData<T, U> {
    data: RefCell<Option<T>>,
    state: Box<dyn Fn(State) -> U>,
    as_of: RefCell<Option<DateTime<Utc>>>,
    fetch: Box<dyn Fn(U) -> Pin<Box<dyn Future<Output = Result<T, anyhow::Error>>>>>,
    cadence: Box<dyn Fn() -> Duration>,
}

impl<T: Clone, U> CachedData<T, U> {
    pub async fn get(&self, state: State) -> Result<Option<T>, anyhow::Error> {
        if self.needs_fetch() {
            let state = (self.state)(state);
            let data = (self.fetch)(state).await.map(|inner| Some(inner))?;
            self.as_of.borrow_mut().replace(Utc::now());
            *self.data.borrow_mut() = data;
        }

        Ok(self.data.borrow().clone())
    }

    pub fn needs_fetch(&self) -> bool {
        if let Some(as_of) = &*self.as_of.borrow() {
            let age = Utc::now() - as_of;
            age > (self.cadence)()
        } else {
            true
        }
    }
}

pub struct DataSource {
    calendar: Option<CachedData<Vec<Event>, CalendarState>>,
    netatmo: Option<CachedData<NetatmoData, NetatmoState>>,
    purple: Option<CachedData<Aqi, PurpleState>>,
    accuweather_daily: Option<CachedData<Vec<DailyForecast>, (LocationState, AccuWeatherState)>>,
    accuweather_hourly: Option<CachedData<Vec<HourlyForecast>, (LocationState, AccuWeatherState)>>,
    birdnet: Option<CachedData<Vec<String>, BirdNetState>>,
}

fn birdnet_cadence() -> Duration {
    Duration::minutes(10)
}

fn birdnet_state(state: State) -> BirdNetState {
    state.birdnet.unwrap().clone()
}

fn fetch_birdnet(birdnet: BirdNetState) -> Pin<Box<dyn Future<Output = Result<Vec<String>, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch birdnet");
        let client = BirdNetClient::new();
        let birds = client.recent_detections(&birdnet).await?;
        Ok(birds)
    })
}

fn calendar_state(state: State) -> CalendarState {
    state.calendar.unwrap().clone()
}

fn calendar_cadence() -> Duration {
    Duration::days(1)
}

fn fetch_calendar(calendar: CalendarState) -> Pin<Box<dyn Future<Output = Result<Vec<Event>, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch calendar");
        let client = calendar::CalendarClient::new();
        let events = client.events(&calendar).await?;
        Ok(events)
    })
}

fn netatmo_cadence() -> Duration {
    Duration::minutes(15)
}

fn netatmo_state(state: State) -> NetatmoState {
    state.netatmo.unwrap().clone()
}

fn fetch_netatmo(netatmo: NetatmoState) -> Pin<Box<dyn Future<Output = Result<NetatmoData, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch netatmo");
        let netatmo_client = netatmo::get_client(&netatmo).await?;
        let netatmo_data = netatmo_client.get_station_data().await?;
        Ok(netatmo_data)
    })
}

fn purple_cadence() -> Duration {
    Duration::hours(2)
}

fn purple_state(state: State) -> PurpleState {
    state.purple.unwrap().clone()
}

fn fetch_purple(purple: PurpleState) -> Pin<Box<dyn Future<Output = Result<Aqi, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch purple");
        let purple_client = purple::PurpleClient::new();
        let aqi = purple_client.get_aqi(&purple).await?;
        Ok(aqi)
    })
}

pub fn accuweather_cadence() -> Duration {
    Duration::minutes(30)
}

fn accuweather_state(state: State) -> (LocationState, AccuWeatherState) {
    (state.location.unwrap().clone(), state.accuweather.unwrap().clone())
}

fn fetch_accuweather_daily_forecast((location, accuweather): (LocationState, AccuWeatherState)
) -> Pin<Box<dyn Future<Output = Result<Vec<DailyForecast>, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch accuweather");
        let client = accuweather::AccuWeatherClient::new();
        let location_key = client.get_location_key(&location, &accuweather).await?;
        let forecast = client.daily_forecast(location_key, &accuweather).await?;
        Ok(forecast)
    })
}

fn fetch_accuweather_hourly_forecast((location, accuweather): (LocationState, AccuWeatherState)
) -> Pin<Box<dyn Future<Output = Result<Vec<HourlyForecast>, anyhow::Error>>>> {
    Box::pin(async move {
        println!("fetch accuweather hourly");
        let client = accuweather::AccuWeatherClient::new();
        let location_key = client.get_location_key(&location, &accuweather).await?;
        let forecast = client.hourly_forecasts(location_key, &accuweather).await?;
        Ok(forecast)
    })
}

impl DataSource {
    pub fn new(state: &State) -> Self {
        Self {
            birdnet: state.birdnet.as_ref().map(|_| CachedData {
                data: RefCell::new(None),
                state: Box::new(birdnet_state),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_birdnet),
                cadence: Box::new(birdnet_cadence),
            }),
            calendar: state.calendar.as_ref().map(|_| CachedData {
                data: RefCell::new(None),
                state: Box::new(calendar_state),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_calendar),
                cadence: Box::new(calendar_cadence),
            }),
            netatmo: state.netatmo.as_ref().map(|_| CachedData {
                data: RefCell::new(None),
                state: Box::new(netatmo_state),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_netatmo),
                cadence: Box::new(netatmo_cadence),
            }),
            purple:state.purple.as_ref().map(|_| CachedData {
                data: RefCell::new(None),
                state: Box::new(purple_state),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_purple),
                cadence: Box::new(purple_cadence),
            }),
            accuweather_daily: state.accuweather.as_ref().map(|_| CachedData {
                data: RefCell::new(None),
                state: Box::new(accuweather_state),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_accuweather_daily_forecast),
                cadence: Box::new(accuweather_cadence),
            }),
            accuweather_hourly: state.accuweather.as_ref().map(|_| CachedData {
                data: RefCell::new(None),
                state: Box::new(accuweather_state),
                as_of: RefCell::new(None),
                fetch: Box::new(fetch_accuweather_hourly_forecast),
                cadence: Box::new(accuweather_cadence),
            }),
        }
    }

    pub async fn get(&self, state: &State) -> Result<DisplayData, anyhow::Error> {
        let now = if state.netatmo.is_some() {
            Some(self.get_now(&state).await?)
        } else {
            None
        };

        let daily_forecast = if state.accuweather.is_some() {
            Some(self.get_daily_forecast(&state).await?)
        } else {
            None
        };

        let hourly_forecast = if state.accuweather.is_some() {
            Some(self.get_hourly_forecast(&state).await?)
        } else {
            None
        };

        let events = if state.calendar.is_some() {
            Some(self.calendar.as_ref().unwrap().get(state.clone()).await?.unwrap_or(vec![]))
        } else {
            Some(vec![])
        };

        let birds = if state.birdnet.is_some() {
            Some(self.birdnet.as_ref().unwrap().get(state.clone()).await?.unwrap_or(vec![]))
        } else {
            Some(vec![])
        };

        Ok(DisplayData {
            //time: Utc::now(),
            now,
            daily_forecast,
            hourly_forecast,
            events,
            birds,
        })
    }

    async fn get_daily_forecast(&self, state: &State) -> Result<Vec<DailyForecast>, anyhow::Error> {
        Ok(self.accuweather_daily.as_ref().unwrap().get(state.clone()).await?.unwrap_or(vec![]))
    }

    async fn get_hourly_forecast(&self, state: &State) -> Result<Vec<HourlyForecast>, anyhow::Error> {
        Ok(self.accuweather_hourly.as_ref().unwrap().get(state.clone()).await?.unwrap_or(vec![]))
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

        if state.netatmo.is_some() {
            if let Some(netatmo) = self.netatmo.as_ref().unwrap().get(state.clone()).await? {
                now_data.temp = netatmo.outside_temp();
                now_data.wind = netatmo.wind();
                now_data.rain = netatmo.rain();
                now_data.humidity = netatmo.humidity();
                now_data.pressure = netatmo.pressure();
            }
        }

        if state.purple.is_some() {
            if let Ok(purple) = self.purple.as_ref().unwrap().get(state.clone()).await {
                now_data.aqi = purple
            }
        }

        Ok(now_data)
    }
}
