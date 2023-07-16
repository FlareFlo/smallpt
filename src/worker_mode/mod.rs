use std::process::exit;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WorkerMode {
	/// Does not compute, only manages work
	Master,
	/// Coordinates work, but also works itself
	MasterWorking,
	/// Operates
	Slave,

	/// Executes without any coordination
	StandAlone,
}

impl Default for WorkerMode {
	fn default() -> Self {
		Self::MasterWorking
	}
}

impl FromStr for WorkerMode {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Master" => Ok(WorkerMode::Master),
			"MasterWorking" => Ok(WorkerMode::MasterWorking),
			"Slave" => Ok(WorkerMode::Slave),
			"StandAlone" => Ok(WorkerMode::StandAlone),

			_ => { Err(()) }
		}
	}
}

impl WorkerMode {
	pub fn from_env() -> Option<Self> {
		let mode = std::env::var("WORKER_MODE");
		match mode {
			Ok(mode) => {
				match Self::from_str(&mode) {
					Ok(x) => {
						Some(x)
					}
					Err(_) => {
						eprintln!("Incorrect WORKER_MODE env var {mode}");
						exit(1);
					}
				}
			}
			Err(e) => {
				eprintln!("{e}, bad env var");
				exit(1);
			}
		}
	}
}