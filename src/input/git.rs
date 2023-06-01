use anyhow::{Context, Result};
use crossterm::style::{style, Stylize};
use inquire::{ui::RenderConfig, Confirm};
use std::{path::PathBuf, process::Command};

use super::{project_location::ProjectLocation, warn_render_config};

pub fn prompt(render_config: &RenderConfig, location: &ProjectLocation) -> Result<Option<PathBuf>> {
	let git_path = which::which("git");

	let git = match git_path {
		Ok(git_path) => {
			let git = Confirm::new("Initialize a new git repository?")
				.with_render_config(*render_config)
				.with_default(true)
				.prompt()?;
			if git {
				git_path
			} else {
				return Ok(None);
			}
		}
		Err(_) => {
			let warn = style("!").red();
			let message = style(format!(
				"Git not found - https://github.com/git-guides/install-git"
			))
			.yellow()
			.bold();
			println!("{warn} {message}");
			println!(
				"  {}",
				style("(git is optional, but recommended)").yellow().dim()
			);
			return Ok(None);
		}
	};
	let closest_git_root = get_closest_git_root(&git, &location.path)?;

	if let Some(closest_git_root) = closest_git_root {
		let init = Confirm::new(
			"Your new project is inside a git repository. Still initialize a new one?",
		)
		.with_render_config(warn_render_config())
		.with_help_message(&format!("{} is a git repository", closest_git_root))
		.with_default(false)
		.prompt()?;
		if !init {
			return Ok(None);
		}
	}

	Ok(Some(git))
	// if
}

fn get_closest_git_root(git: &PathBuf, directory: &PathBuf) -> Result<Option<String>> {
	let closest_existing_dir = directory
		.parent()
		.unwrap_or(directory)
		.ancestors()
		.find(|dir| dir.exists())
		.context("Could not find closest existing directory")?;

	let output = Command::new(&git)
		.current_dir(closest_existing_dir)
		.arg("rev-parse")
		.arg("--show-toplevel")
		.output()
		.context("Failed to execute 'git rev-parse --show-toplevel'")?;

	if !output.status.success() {
		return Ok(None);
	}

	let output = String::from_utf8(output.stdout)
		.context("'git rev-parse --show-toplevel' output is invalid UTF-8")?;

	Ok(Some(output.trim().to_string()))
}
