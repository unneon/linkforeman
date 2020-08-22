use anyhow::{Context, Result};
use dirs::home_dir;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
pub struct Config {
	links: HashMap<String, LinkEntry>,
}

#[derive(Debug, Deserialize)]
struct LinkEntry {
	#[serde(flatten)]
	link: Link,
	#[serde(default)]
	recursive: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
	src: PathBuf,
	dest: PathBuf,
}

impl Config {
	pub fn links(&self) -> impl Iterator<Item=Link>+'_ {
		self.links.values().flat_map(expand_link_entry)
	}
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

fn expand_link_entry(link: &LinkEntry) -> Vec<Link> {
	if !link.recursive {
		return vec![link.link.clone()];
	}

	let mut links = Vec::new();
	for entry in WalkDir::new(&link.link.src) {
		if let Ok(entry) = entry {
			if entry.file_type().is_file() {
				if let Ok(relative) = entry.path().strip_prefix(&link.link.src) {
					let src = entry.path().to_owned();
					let dest = link.link.dest.join(relative);
					links.push(Link { src, dest });
				}
			}
		}
	}
	links
}
