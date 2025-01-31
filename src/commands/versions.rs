use crate::configuration::ApplicationName;
use crate::prelude::*;
use crate::{applications, logging};
use std::process::ExitCode;

pub(crate) fn versions(args: &Args) -> Result<ExitCode> {
  let apps = &applications::all();
  let app = apps.lookup(args.app_name.as_str())?;
  let log = logging::new(args.verbose);
  let versions = app.installable_versions(args.amount, log)?;
  println!("{} is available in these versions:", args.app_name);
  for version in versions {
    println!("- {version}");
  }
  Ok(ExitCode::SUCCESS)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Args<'a> {
  pub(crate) app_name: ApplicationName<'a>,
  pub(crate) amount: usize,
  pub(crate) verbose: bool,
}
