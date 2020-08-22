mod tree;

use crate::{
	config::{Config, Link}, status::tree::Tree
};
use anyhow::Result;

enum Status {
	Active,
	DoesNotExist,
	WrongType,
	WrongDestination,
}

struct Row {
	status: Status,
	destination: String,
}

pub fn run(config: &Config) -> Result<()> {
	let mut tree = Tree::new();
	for link in config.links() {
		let components = link.components()?;
		let components: Vec<_> = components.iter().map(String::as_str).collect();
		let destination = link.destination()?.display().to_string();
		let row = Row { status: query_status(&link)?, destination };
		tree.add(&components, row)?;
	}
	println!("{}", std::env::current_dir().unwrap().display());
	tree.render("", display_status);
	Ok(())
}

fn query_status(link: &Link) -> Result<Status> {
	let destination = link.destination()?;
	Ok(if !destination.exists() {
		Status::DoesNotExist
	} else {
		let metadata = destination.symlink_metadata()?;
		if !metadata.file_type().is_symlink() {
			Status::WrongType
		} else if destination.canonicalize()? != link.source()?.canonicalize()? {
			Status::WrongDestination
		} else {
			Status::Active
		}
	})
}

fn display_status(name: &str, row: &Row) -> String {
	let (color, message) = match row.status {
		Status::Active => (32, "active"),
		Status::DoesNotExist => (31, "does not exist"),
		Status::WrongDestination => (31, "wrong destination"),
		Status::WrongType => (31, "wrong type"),
	};
	format!("{}, \x1B[1;{}m● {}\x1B[0m at {}", name, color, message, row.destination)
}
