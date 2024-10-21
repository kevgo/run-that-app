use crate::config::AppName;
use crate::prelude::*;
use crate::{apps, logger};
use std::process::ExitCode;

pub fn versions(args: &Args) -> Result<ExitCode> {
  let apps = &apps::all();
  let app = apps.lookup(&args.app_name)?;
  let log = logger::new(args.verbose);
  let versions = app.installable_versions(args.amount, log)?;
  println!("{} is available in these versions:", args.app_name);
  for version in versions {
    println!("- {version}");
  }
  Ok(ExitCode::SUCCESS)
}

#[derive(Debug, PartialEq)]
pub struct Args {
  pub app_name: AppName,
  pub amount: usize,
  pub verbose: bool,
}
