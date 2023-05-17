use clap::Parser;
use anyhow::{Result, Ok};
use roni::{self, config::config::Config, cmd};

pub fn main() -> Result<()> {
    let opts: Config = cmd::opts::Opts::parse().try_into()?;


    // TODO: based on the operation, do core logic
    print!("opts: {:?}", opts);
    Ok(())
}