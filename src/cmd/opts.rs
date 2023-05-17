use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "roni", version = "1.0", author = "Shahen H.")]
pub struct Opts {
  pub args: Vec<String>,
}