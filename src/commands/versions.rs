use crate::applications::{ApplicationName, Apps};
use crate::error::Result;
use crate::logging;
use std::process::ExitCode;

pub fn versions(args: &Args, apps: &Apps) -> Result<ExitCode> {
  let app = apps.lookup(&args.app_name)?;
  let log = logging::new(args.verbose);
  let versions = app.installable_versions(args.amount, log)?;
  println!("{} is available in these versions:", args.app_name);
  for version in versions {
    println!("- {version}");
  }
  Ok(ExitCode::SUCCESS)
}

#[derive(Debug, PartialEq)]
pub struct Args {
  pub app_name: ApplicationName,
  pub amount: usize,
  pub verbose: bool,
}
