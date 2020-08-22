use clap::Clap;

/// Simple application to keep track of your config symlinks.
#[derive(Clap, Debug)]
pub enum Opts {
	/// Display the status of all managed links.
	Status,
}
