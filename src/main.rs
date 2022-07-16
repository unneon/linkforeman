use anyhow::Result;
use clap::Parser;
use cli::Opts;

mod cli;
mod config;
mod status;

fn main() -> Result<()> {
	let opts: Opts = Opts::parse();
	let config = config::load()?;
	match opts {
		Opts::Status => status::run(&config),
	}
}
