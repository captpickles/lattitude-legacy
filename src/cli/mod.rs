use crate::data::DataSource;
use crate::display::Display;
use crate::paint::Paint;
use clap::{Args, Parser, Subcommand};
use std::time::Duration;
use chrono::Utc;
use crate::birdnet::BirdNetClient;
use crate::state::state;

#[derive(Debug, Clone, Parser)]
#[command(
author,
version = env ! ("CARGO_PKG_VERSION"),
about = "L'åttitüdé",
long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Clear(ClearCommand),
    Unbox(UnboxCommand),
    Splash(SplashCommand),
    Screen(ScreenCommand),
    Loop(LoopCommand),
    Test(TestCommand),
}

#[derive(Args, Debug, Clone)]
#[command(about = "Clear the screen", args_conflicts_with_subcommands = true)]
pub struct ClearCommand {}

impl ClearCommand {
    pub async fn run<P: Paint>(&self, paint: &mut P) -> Result<(), anyhow::Error> {
        let mut display = Display::new(paint);
        display.draw_clear_screen()?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
#[command(
about = "Draw the unboxing screen",
args_conflicts_with_subcommands = true
)]
pub struct UnboxCommand {}

impl UnboxCommand {
    pub async fn run<P: Paint>(&self, paint: &mut P) -> Result<(), anyhow::Error> {
        let mut display = Display::new(paint);
        display.draw_unbox_screen()?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
#[command(
about = "Draw the splash screen",
args_conflicts_with_subcommands = true
)]
pub struct SplashCommand {}

impl SplashCommand {
    pub async fn run<P: Paint>(&self, paint: &mut P) -> Result<(), anyhow::Error> {
        let mut display = Display::new(paint);
        display.draw_splash_screen()?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
#[command(about = "Draw the data screen", args_conflicts_with_subcommands = true)]
pub struct ScreenCommand {}

impl ScreenCommand {
    pub async fn run<P: Paint>(&self, paint: &mut P) -> Result<(), anyhow::Error> {
        let state = state();
        let ds = DataSource::new(&state);
        let data = ds.get(&state).await?;

        let mut display = Display::new(paint);
        display.draw_data_screen(&data, Utc::now())?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
#[command(about = "Loop it all", args_conflicts_with_subcommands = true)]
pub struct LoopCommand {}

impl LoopCommand {
    pub async fn run<P: Paint>(&self, paint: &mut P) -> Result<(), anyhow::Error> {
        let state = state();
        let mut display = Display::new(paint);
        let _ = display.draw_clear_screen();

        let mut display = Display::new(paint);
        let _ = display.draw_splash_screen();

        let ds = DataSource::new(&state);

        let mut prev_data = None;

        loop {

            let data = ds.get(&state).await?;
            /*
            if let Some(prev_data) = &prev_data {
                if *prev_data == data {
                    println!("no redraw");
                    continue;
                }
            }
            println!("redraw");

             */
            let mut display = Display::new(paint);
            display.draw_data_screen(&data, Utc::now())?;
            prev_data.replace(data);
            tokio::time::sleep(Duration::from_secs(60)).await;
            /*
            let mut display = Display::new(paint);
            display.draw_header_only(Utc::now())?;
            tokio::time::sleep(Duration::from_secs(1)).await;
             */
        }
    }
}


#[derive(Args, Debug, Clone)]
#[command(about = "Run whatever it is you're testing", args_conflicts_with_subcommands = true)]
pub struct TestCommand;

impl TestCommand {
    pub async fn run<P: Paint>(&self, _paint: &mut P) -> Result<(), anyhow::Error> {
        let state = state();
        let birdnet = state.birdnet.as_ref().unwrap();
        let client = BirdNetClient::new();
        client.recent_detections(birdnet).await?;
        Ok(())
    }
}