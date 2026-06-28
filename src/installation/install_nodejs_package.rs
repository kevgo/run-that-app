use crate::applications::{AppDefinition, Apps, Npm};
use crate::commands::RunArgs;
use crate::error::{Result, UserError};
use crate::installation::Outcome;
use crate::{Version, commands, logging};
use std::path::Path;
use std::{env, fs};

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

  // run "npm install" inside the app folder so that "node_modules" is created next to the "package.json" file
  let previous_dir = env::current_dir().map_err(|err| UserError::CannotDetermineCurrentDirectory(err.to_string()))?;
  env::set_current_dir(app_folder).map_err(|err| UserError::CannotDetermineCurrentDirectory(err.to_string()))?;
  // npm is distributed together with NodeJS, so we install it at the latest available NodeJS version
  let npm = Npm {};
  let npm_version = npm.latest_installable_version(logging::new(false))?;
  let result = commands::run(
    RunArgs {
      app_name: npm.name(),
      app_args: vec!["install".to_string()],
      version: Some(npm_version),
      optional,
      from_source: false,
      include_apps: vec![],
      verbose: false,
      error_on_output: false,
    },
    apps,
  );
  env::set_current_dir(&previous_dir).map_err(|err| UserError::CannotDetermineCurrentDirectory(err.to_string()))?;
  result?;

  Ok(Outcome::Installed)
}
