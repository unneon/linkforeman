use anyhow::{bail, ensure, Result};
use std::{collections::BTreeMap, fmt};

#[derive(Debug, Eq, PartialEq)]
pub enum Tree<T> {
	Leaf { object: T },
	Directory { children: BTreeMap<String, Tree<T>> },
}

impl<T> Tree<T> {
	pub fn new() -> Self {
		Tree::Directory { children: BTreeMap::new() }
	}

	pub fn add(&mut self, path: &[&str], object: T) -> Result<()> {
		ensure!(path.len() > 0, "root object must be a directory");
		let mut directory = self.get_root_mut();
		for component in &path[..path.len() - 1] {
			directory = get_subdirectory(directory, *component)?;
		}
		ensure!(
			!directory.contains_key(path[path.len() - 1]),
			"object with given path already exists"
		);
		directory.insert(path[path.len() - 1].to_owned(), Tree::Leaf { object });
		Ok(())
	}

	fn get_root(&self) -> &BTreeMap<String, Tree<T>> {
		match self {
			Tree::Leaf { .. } => panic!("root object is not a directory"),
			Tree::Directory { children } => children,
		}
	}

	fn get_root_mut(&mut self) -> &mut BTreeMap<String, Tree<T>> {
		match self {
			Tree::Leaf { .. } => panic!("root object is not a directory"),
			Tree::Directory { children } => children,
		}
	}

	pub fn render<Fmt: fmt::Display>(&self, prefix: &str, display: impl Fn(&str, &T) -> Fmt) {
		render(self.get_root(), prefix, &display)
	}
}

fn get_subdirectory<'a, T>(
	children: &'a mut BTreeMap<String, Tree<T>>,
	name: &str,
) -> Result<&'a mut BTreeMap<String, Tree<T>>> {
	let entry = children
		.entry(name.to_owned())
		.or_insert_with(|| Tree::Directory { children: BTreeMap::new() });
	match entry {
		Tree::Leaf { .. } => bail!("file {} already exists in this directory", name),
		Tree::Directory { children } => Ok(children),
	}
}

fn render<T, Fmt: fmt::Display>(
	tree: &BTreeMap<String, Tree<T>>,
	prefix: &str,
	display: &impl Fn(&str, &T) -> Fmt,
) {
	for (i, (name, kid)) in tree.iter().enumerate() {
		let intersection = if i == tree.len() - 1 { '└' } else { '├' };
		match kid {
			Tree::Leaf { object } => {
				let text = (*display)(&name, object);
				println!("{}{}─{}", prefix, intersection, text)
			}
			Tree::Directory { children } => {
				let subpipe = if i == tree.len() - 1 { ' ' } else { '│' };
				let subprefix = format!("{}{} ", prefix, subpipe);
				println!("{}{}─{}", prefix, intersection, name);
				render(children, &subprefix, display)
			}
		}
	}
}
