use crate::applications::{AppDefinition, Apps, Npm};
use crate::commands::RunArgs;
use crate::error::{Result, UserError};
use crate::installation::Outcome;
use crate::{Version, commands};
use std::fs;
use std::path::Path;

pub fn run(package_name: &str, app_folder: &Path, version: &Version, optional: bool, apps: &Apps) -> Result<Outcome> {
  // create the package.json file
  let filepath = app_folder.join("package.json");
  let content = format!(
    r#"{{
  "dependencies": {{
    "{package_name}": "{version}"
  }}
}}"#,
  );
  fs::write(&filepath, content).map_err(|err| UserError::CannotCreateFile {
    filepath: filepath.to_string_lossy().to_string(),
    err: err.to_string(),
  })?;

  // run "npm install"
  let npm = Npm {};
  commands::run(
    RunArgs {
      app_name: npm.name().into(),
      app_args: vec!["install".to_string()],
      version: None,
      optional,
      from_source: false,
      include_apps: vec![],
      verbose: false,
      error_on_output: false,
    },
    apps,
  )?;

  Ok(Outcome::Installed)
}
