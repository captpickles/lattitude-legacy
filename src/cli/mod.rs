use clap::{Args, Parser, Subcommand};
use crate::data::DataSource;
use crate::display::Display;
use crate::paint::Paint;

#[derive(Debug, Clone, Parser)]
#[command(
    author,
    version = env!("CARGO_PKG_VERSION"),
    about = "L'åttitüdé",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Clear(ClearCommand),
    Splash(SplashCommand),
    Screen(ScreenCommand),
}

#[derive(Args, Debug, Clone)]
#[command(about = "Clear the screen", args_conflicts_with_subcommands = true)]
pub struct ClearCommand {

}

impl ClearCommand {
    pub async fn run<P: Paint>(&self, paint: &mut P) -> Result<(), anyhow::Error>{
        let mut display = Display::new(paint);
        display.draw_clear_screen()?;
        Ok(())
    }

}


#[derive(Args, Debug, Clone)]
#[command(about = "Draw the splash screen", args_conflicts_with_subcommands = true)]
pub struct SplashCommand {

}

impl SplashCommand {
    pub async fn run<P: Paint>(&self, paint: &mut P) -> Result<(), anyhow::Error>{
        let mut display = Display::new(paint);
        display.draw_splash_screen()?;
        Ok(())
    }

}



#[derive(Args, Debug, Clone)]
#[command(about = "Draw the data screen", args_conflicts_with_subcommands = true)]
pub struct ScreenCommand {

}

impl ScreenCommand {
    pub async fn run<P: Paint>(&self, paint: &mut P) -> Result<(), anyhow::Error>{
        let ds = DataSource::new();
        let data = ds.get().await?;

        let mut display = Display::new(paint);
        display.draw_data_screen(data)?;
        Ok(())
    }
}