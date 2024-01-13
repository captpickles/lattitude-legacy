use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub location: LocationState,
    pub netatmo: NetatmoState,
    pub purple: PurpleState,
    pub accuweather: AccuWeatherState,
    pub calendar: CalendarState,
    pub birdnet: BirdNetState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationState {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetatmoState {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PurpleState {
    pub api_key: String,
    pub sensor_index: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccuWeatherState {
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarState {
    pub urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BirdNetState {
    pub token: String,
}

static STATE: RwLock<Option<State>> = RwLock::new(None);

pub fn state() -> State {
    if STATE.read().unwrap().is_none() {
        let mut config = File::open("lattitude.toml").expect("Missing lattitude.toml file!");
        let mut data = String::new();
        let _ = config.read_to_string(&mut data);

        let state: State = toml::from_str(&data).unwrap();

        STATE.write().unwrap().replace(state);
    }

    STATE.read().unwrap().clone().unwrap()
}

pub fn update_state<F: FnOnce(&mut State)>(updater: F) {
    let mut state = state();
    updater(&mut state);

    let toml = toml::to_string_pretty(&state).unwrap();
    let mut config = File::create("latitude.toml").unwrap();

    config.write_all(toml.as_bytes()).unwrap();

    STATE.write().unwrap().replace(state);
}

#[cfg(test)]
mod test {
    use crate::state::{state, update_state, State};
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn load_state() {
        let mut config = File::open("latitude.toml").unwrap();
        let mut data = String::new();
        config.read_to_string(&mut data);

        let state: State = toml::from_str(&data).unwrap();

        println!("{state:#?}")
    }

    #[test]
    fn lazy_state() {
        let state = state();
        println!("{:#?}", state);
    }

    #[test]
    fn update() {
        /*
        update_state( |state| {
            state.netatmo.refresh_token = "taco".to_string();
        });

        let state = state();
        println!("{:#?}", state);

         */
    }
}
