use clap::Parser;

/// Simple application to keep track of your config symlinks.
#[derive(Parser, Debug)]
pub enum Opts {
	/// Display the status of all managed links.
	Status,
}
