use crate::applications::Apps;
use crate::commands::run::load_or_install_app;
use crate::configuration::RequestedVersions;
use crate::error::Result;
use crate::{configuration, logging};
use std::process::ExitCode;

pub(crate) fn install_all(apps: &Apps, verbose: bool) -> Result<ExitCode> {
  let config_file = configuration::File::load(apps)?;
  let log = logging::new(verbose);
  let apps_to_install = config_file.apps;
  for app_version in apps_to_install {
    let app_def = apps.lookup(&app_version.app_name)?;
    let version = app_def.latest_installable_version(log)?;
    let requested_versions = RequestedVersions::from(vec![RequestedVersion::Yard(version)]);
    let Some(executable_call) = load_or_install_app(app_def, requested_versions, args.optional, args.from_source, &ctx)? else {
      if args.optional {
        continue;
      }
    };
  }
  Ok(ExitCode::SUCCESS)
}
