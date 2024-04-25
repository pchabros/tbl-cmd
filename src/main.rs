use app::App;
use clap::Parser;
use cli::Cli;
use std::error::Error;

mod app;
mod cli;
mod table;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::init()?;
    let input = Cli::parse().input;
    App::new(&input).run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
