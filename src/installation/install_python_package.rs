use crate::applications::{AppDefinition, Apps, Uv};
use crate::commands::RunArgs;
use crate::error::{Result, UserError};
use crate::installation::Outcome;
use crate::{Version, commands};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// name of the folder containing the Python virtual environment
const VENV: &str = ".venv";

pub fn run(app: &dyn AppDefinition, package_name: &str, app_folder: &Path, version: &Version, optional: bool, apps: &Apps) -> Result<Outcome> {
  // create a virtual environment that keeps working after it gets moved into its final location in the yard
  run_uv(&["venv", "--quiet", "--relocatable", VENV], app, app_folder, optional, apps)?;

  // install the Python package into the virtual environment
  let requirement = format!("{package_name}=={version}");
  run_uv(&["pip", "install", "--python", VENV, &requirement], app, app_folder, optional, apps)?;

  Ok(Outcome::Installed)
}

/// provides the possible locations of the given script inside the Python virtual environment in the given folder
pub fn executable_paths(folder: &Path, script: &str) -> Vec<PathBuf> {
  let venv = folder.join(VENV);
  vec![venv.join("bin").join(script), venv.join("Scripts").join(format!("{script}.exe"))]
}

/// runs uv with the given arguments in the given folder
fn run_uv(args: &[&str], needed_by: &dyn AppDefinition, cwd: &Path, optional: bool, apps: &Apps) -> Result<()> {
  let uv = Uv {};
  let exit_code = commands::run(
    RunArgs {
      app_name: uv.name(),
      app_args: args.iter().map(ToString::to_string).collect(),
      version: None,
      optional,
      from_source: false,
      include_apps: vec![],
      verbose: false,
      error_on_output: false,
      cwd: Some(cwd.to_path_buf()),
    },
    apps,
  );
  match exit_code {
    Ok(ExitCode::SUCCESS) => Ok(()),
    Ok(_) => Err(UserError::UvInstallFailed),
    Err(UserError::NoVersionsFound { app: runtime }) if runtime == uv.name() => Err(UserError::MissingRuntime {
      runtime,
      needed_by: needed_by.name(),
      script: None,
      searched_dirs: vec![],
    }),
    Err(err) => Err(err),
  }
}

#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};

  #[test]
  fn executable_paths() {
    let have = super::executable_paths(Path::new("folder"), "pyright");
    let want = vec![
      PathBuf::from("folder").join(".venv").join("bin").join("pyright"),
      PathBuf::from("folder").join(".venv").join("Scripts").join("pyright.exe"),
    ];
    assert_eq!(have, want);
  }
}
