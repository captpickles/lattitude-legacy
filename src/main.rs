use serde::Deserialize;
use crate::accuweather::AccuWeatherClient;
use crate::calendar::CalendarClient;
use crate::data::DataSource;
use crate::display::Display;

mod netatmo;
mod purple;
mod display;
mod graphics;
mod font;
mod data;
pub mod art;
mod accuweather;
mod calendar;
mod state;

//pub const LAT: &str ="36.949817";
//pub const LON: &str = "-81.077840";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();


    let ds = DataSource::new();
    let data = ds.get().await?;

    let display = Display::new();
    display.draw_data_screen(data)?;
    //display.draw_splash_screen()?;



    Ok(())
}
