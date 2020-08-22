use anyhow::{Context, Result};
use dirs::home_dir;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
	pub links: HashMap<String, Link>,
}

#[derive(Debug, Deserialize)]
pub struct Link {
	src: PathBuf,
	dest: PathBuf,
}

impl Link {
	pub fn source(&self) -> Result<PathBuf> {
		Ok(current_dir()?.join(&self.src))
	}

	pub fn components(&self) -> Result<Vec<String>> {
		let source = self.source()?;
		let relative = source.strip_prefix(current_dir()?)?;
		relative
			.components()
			.map(|comp| Ok(comp.as_os_str().to_str().context("path is not utf8")?.to_owned()))
			.collect()
	}

	pub fn destination(&self) -> Result<PathBuf> {
		Ok(home_dir().context("querying home directory failed")?.join(&self.dest))
	}
}

pub fn load() -> Result<Config> {
	let file = std::fs::read(".linkforeman.toml")?;
	Ok(toml::de::from_slice(&file)?)
}

fn current_dir() -> Result<PathBuf> {
	Ok(std::env::current_dir()?)
}
