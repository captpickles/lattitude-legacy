use clap::Parser;
use serde::Deserialize;
use crate::accuweather::AccuWeatherClient;
use crate::calendar::CalendarClient;
use crate::cli::{Cli, Command};
use crate::data::DataSource;
use crate::display::Display;
use crate::paint::{NoOpPaint, Paint};

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
mod cli;
mod paint;

//pub const LAT: &str ="36.949817";
//pub const LON: &str = "-81.077840";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let cli = Cli::parse();

    let mut paint = new_paint();

    match cli.command {
        Command::Clear(inner) => {
            inner.run(&mut paint).await?;
        }
        Command::Splash(inner) => {
            inner.run(&mut paint).await?;
        }
        Command::Screen(inner) => {
            inner.run(&mut paint).await?;
        }
        Command::Loop(inner) => {
            inner.run(&mut paint).await?;
        }
    }

    /*
    println!("{:#?}", cli);


    let ds = DataSource::new();
    let data = ds.get().await?;

    let display = Display::new();
    display.draw_data_screen(data)?;
    //display.draw_splash_screen()?;

     */



    Ok(())
}

#[cfg(feature = "linux-embedded-hal")]
pub fn new_paint() -> impl Paint {
    use crate::paint::epd::EpdPaint;
    EpdPaint::new()
}

#[cfg(not(feature="linux-embedded-hal"))]
pub fn new_paint() -> impl Paint {
    NoOpPaint
}
