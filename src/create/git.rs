use std::process::{Command, Stdio};

use anyhow::Error;

use crate::{create::log_step_start, input::UserInput};

use super::log_step_end;
macro_rules! run_git {
	(
		$git:ident,
		$input:ident,
		$step:expr
	) => {
		let args = match $step {
			GitStep::Init => vec!["init"],
			GitStep::Add => vec!["add", "."],
			GitStep::Commit => vec!["commit", "-m", crate::utils::INITIAL_COMMIT],
		};
		let status = Command::new(&$git)
			.args(args)
			.current_dir(&$input.location.path)
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.status();
		match status {
			Ok(status) => {
				if !status.success() {
					return Err(($step, anyhow::anyhow!("Could not create git repository")));
				}
			}
			Err(e) => {
				return Err(($step, anyhow::Error::from(e)));
			}
		}
	};
}

#[derive(Debug)]
pub enum GitStep {
	Init,
	Add,
	Commit,
}

pub fn create_repo(input: &UserInput) -> Result<(), (GitStep, Error)> {
	if input.git.is_none() {
		return Ok(());
	}
	let git = input.git.as_ref().unwrap();

	let start = log_step_start("Creating git repository...\n");

	run_git!(git, input, GitStep::Init);

	run_git!(git, input, GitStep::Add);

	run_git!(git, input, GitStep::Commit);

	println!();

	log_step_end(start);

	Ok(())
}
