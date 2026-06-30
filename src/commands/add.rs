use crate::applications::{AppDefinition, Apps};
use crate::error::Result;
use crate::{configuration, logging};
use std::process::ExitCode;

pub fn add(args: AddArgs, apps: &Apps) -> Result<ExitCode> {
  let log = logging::new(args.verbose);
  let version = args.app.latest_installable_version(log)?;
  if let Some(config_file) = configuration::File::read(apps)? {
    config_file.add(args.app.name(), version.clone())?;
  } else {
    configuration::File::create(&args.app.name(), &version.clone())?;
  }
  eprintln!("added {}@{} to {}", args.app.name(), &version, configuration::FILE_NAME);
  Ok(ExitCode::SUCCESS)
}

/// named arguments for the [`add`] command
#[derive(Debug, PartialEq)]
pub struct AddArgs<'a> {
  pub app: &'a Box<dyn AppDefinition>,
  pub verbose: bool,
}
