use crate::cli::{Cli, Command};
use crate::paint::{NoOpPaint, Paint};
use clap::Parser;

mod accuweather;
pub mod art;
mod calendar;
mod cli;
mod data;
mod display;
mod font;
mod graphics;
mod netatmo;
mod paint;
mod purple;
mod state;
mod birdnet;

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
        Command::Unbox(inner) => {
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
        Command::Test(inner) => {
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

#[cfg(not(feature = "linux-embedded-hal"))]
pub fn new_paint() -> impl Paint {
    NoOpPaint
}
