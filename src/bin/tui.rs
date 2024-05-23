use std::{error::Error, time::Duration};

use argh::FromArgs;

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "200")]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    notes::run(tick_rate)?;
    Ok(())
}
