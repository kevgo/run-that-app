use crate::applications::{AppDefinition, Apps, NodeJS, Npm};
use crate::commands::RunArgs;
use crate::error::{Result, UserError};
use crate::installation::Outcome;
use crate::{Version, commands, logging};
use std::fs;
use std::path::Path;

pub fn run(package_name: &str, app_folder: &Path, version: &Version, optional: bool, apps: &Apps) -> Result<Outcome> {
  // create the package.json file
  let filepath = app_folder.join("package.json");
  let content = format!(
    r#"
{{
  "dependencies": {{
    "{package_name}": "{version}"
  }}
}}"#,
  );
  fs::write(&filepath, &content[1..]).map_err(|err| UserError::CannotCreateFile {
    filename: filepath.to_string_lossy().to_string(),
    err: err.to_string(),
  })?;

  // run "npm install" inside the app folder so that "node_modules" is created next to the "package.json" file
  // npm is distributed together with NodeJS, so we install it at the latest available NodeJS version
  let npm = Npm {};
  let npm_version = npm.latest_installable_version(logging::new(false))?;
  let nodejs = NodeJS {};
  commands::run(
    RunArgs {
      app_name: npm.name(),
      app_args: vec!["install".to_string()],
      version: Some(npm_version),
      optional,
      from_source: false,
      include_apps: vec![nodejs.name()],
      verbose: false,
      error_on_output: false,
      cwd: Some(app_folder.to_path_buf()),
    },
    apps,
  )?;

  Ok(Outcome::Installed)
}
