use crate::applications::{AnalyzeResult, AppDefinition, ApplicationName, Apps, NodeJS, carrier};
use crate::configuration::{self, AppVersions, RequestedVersion, RequestedVersions, Version};
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::executables::{ExecutableCall, ExecutableCallDefinition, RunMethod};
use crate::filesystem::find_global_install;
use crate::installation::{self, Outcome};
use crate::logging::{self, Event};
use crate::yard::Yard;
use crate::{platform, subshell, yard};
use ahash::AHashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn run(args: RunArgs, apps: &Apps) -> Result<ExitCode> {
  let app_to_run = apps.lookup(&args.app_name)?;
  let log = logging::new(args.verbose);
  let platform = platform::detect(log)?;
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let config_file = configuration::File::load(apps)?;
  let ctx = RuntimeContext {
    platform,
    yard: &yard,
    config_file: &config_file,
    log,
  };
  let include_app_versions = config_file.lookup_many(args.include_apps);
  let include_apps = load_or_install_apps(&include_app_versions, apps, args.optional, args.from_source, &ctx)?;
  let requested_versions = RequestedVersions::determine(&args.app_name, args.version.as_ref(), &config_file)?;
  let Some(executable_call) = load_or_install_app(app_to_run, &requested_versions, args.optional, args.from_source, &ctx, apps)? else {
    if args.optional {
      return Ok(ExitCode::SUCCESS);
    }
    return Err(UserError::UnsupportedPlatform);
  };
  let cwd = args.cwd.as_deref();
  if args.error_on_output {
    let (executable, args) = executable_call.with_args(args.app_args);
    subshell::detect_output(&executable, &args, &include_apps, cwd)
  } else {
    let (executable, args) = executable_call.with_args(args.app_args);
    subshell::stream_output(&executable, &args, &include_apps, cwd)
  }
}

/// data needed to run an executable
#[derive(Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct RunArgs {
  /// name of the app to execute
  pub app_name: ApplicationName,

  /// possible versions of the app to execute
  pub version: Option<Version>,

  /// arguments to call the app with
  #[allow(clippy::struct_field_names)]
  pub app_args: Vec<String>,

  /// if true, any output produced by the app is equivalent to an exit code > 0
  pub error_on_output: bool,

  /// if true, install only from source
  pub from_source: bool,

  /// other applications to include into the PATH
  pub include_apps: Vec<ApplicationName>,

  /// whether it's okay to not run the app if it cannot be installed
  pub optional: bool,

  pub verbose: bool,

  /// optional working directory in which to execute the app
  pub cwd: Option<PathBuf>,
}

pub fn load_or_install_apps(
  app_versions: &Vec<AppVersions>,
  apps: &Apps,
  optional: bool,
  from_source: bool,
  ctx: &RuntimeContext,
) -> Result<Vec<ExecutableCall>> {
  let mut result = vec![];
  for app_version in app_versions {
    let app = apps.lookup(&app_version.app_name)?;
    if let Some(executable_call) = load_or_install_app(app, &app_version.versions, optional, from_source, ctx, apps)? {
      result.push(executable_call);
    }
  }
  Ok(result)
}

// TODO: convert to named arguments
pub fn load_or_install_app(
  app_definition: &dyn AppDefinition,
  requested_versions: &RequestedVersions,
  optional: bool,
  from_source: bool,
  ctx: &RuntimeContext,
  apps: &Apps,
) -> Result<Option<ExecutableCall>> {
  for requested_version in requested_versions {
    if let Some(executable_call) = load_or_install(app_definition, requested_version, optional, from_source, ctx, apps)? {
      return Ok(Some(executable_call));
    }
  }
  Ok(None)
}

// TODO: convert to named arguments
fn load_or_install(
  app_definition: &dyn AppDefinition,
  requested_version: &RequestedVersion,
  optional: bool,
  from_source: bool,
  ctx: &RuntimeContext,
  apps: &Apps,
) -> Result<Option<ExecutableCall>> {
  match requested_version {
    RequestedVersion::Path(version) => {
      if let Some(executable_call_def) = load_from_path(app_definition, version, ctx)?
        && let Some(app_folder) = executable_call_def.executable.clone().as_path().parent()
        && let Some(executable_call) = executable_call_def.into_executable_call(app_folder)
      {
        return Ok(Some(executable_call));
      }
      Ok(None)
    }
    RequestedVersion::Yard(version) => load_or_install_from_yard(app_definition, version, optional, from_source, ctx, apps),
  }
}

// finds the app in the PATH and verifies it has the correct version
fn load_from_path(app_to_run: &dyn AppDefinition, range: &semver::VersionReq, ctx: &RuntimeContext) -> Result<Option<ExecutableCallDefinition>> {
  let (app_to_install, executable_name, executable_args) = carrier(app_to_run, &Version::from(""), ctx.platform);
  let executable_filename = executable_name.platform_path(ctx.platform.os);
  let Some(executable) = find_global_install(&executable_filename, ctx.log) else {
    (ctx.log)(Event::GlobalInstallNotFound);
    return Ok(None);
  };
  match app_to_install.analyze_executable(&executable, ctx.log)? {
    AnalyzeResult::NotIdentified { output: _ } => {
      (ctx.log)(Event::GlobalInstallNotIdentified);
      Ok(None)
    }
    AnalyzeResult::IdentifiedButUnknownVersion if range.to_string() == "*" => {
      (ctx.log)(Event::GlobalInstallMatchingVersion { range, version: None });
      Ok(Some(ExecutableCallDefinition {
        executable,
        args: executable_args,
      }))
    }
    AnalyzeResult::IdentifiedButUnknownVersion => {
      (ctx.log)(Event::GlobalInstallMismatchingVersion { range, version: None });
      Ok(None)
    }
    AnalyzeResult::IdentifiedWithVersion(version) if range.matches(&version.semver()?) => {
      (ctx.log)(Event::GlobalInstallMatchingVersion {
        range,
        version: Some(&version),
      });
      Ok(Some(ExecutableCallDefinition {
        executable,
        args: executable_args,
      }))
    }
    AnalyzeResult::IdentifiedWithVersion(version) => {
      (ctx.log)(Event::GlobalInstallMismatchingVersion {
        range,
        version: Some(&version),
      });
      Ok(None)
    }
  }
}

// TODO: convert to named arguments
fn load_or_install_from_yard(
  app_definition: &dyn AppDefinition,
  version: &Version,
  optional: bool,
  from_source: bool,
  ctx: &RuntimeContext,
  apps: &Apps,
) -> Result<Option<ExecutableCall>> {
  // A NodeJS package is installed into its own app folder via "npm install" and then executed through NodeJS.
  // This needs to install two separate apps (the package and NodeJS itself, each with their own version),
  // so it cannot go through the generic single-app installation flow below.
  if let RunMethod::NodeJS { package: _ } = app_definition.run_method(version, ctx.platform) {
    return load_or_install_nodejs_package(app_definition, version, optional, from_source, ctx, apps);
  }
  let (app_to_install, executable_name, executable_args) = carrier(app_definition, version, ctx.platform);
  let app_name = app_to_install.name();
  // try to load the app
  if let Some((executable, bin_folder)) = ctx.yard.load_executable(app_to_install.as_ref(), &executable_name, version, ctx) {
    let app_folder = ctx.yard.app_folder(&app_name, version);
    let args = executable_args.locate(&app_folder, &bin_folder)?;
    return Ok(Some(ExecutableCall { executable, args }));
  }
  // app not installed --> check if uninstallable
  if ctx.yard.is_not_installable(&app_name, version) {
    return Ok(None);
  }
  // app not installed and installable --> try to install
  match installation::any(app_to_install.as_ref(), version, optional, from_source, ctx, apps)? {
    Outcome::Installed => {} // we'll load it below
    Outcome::NotInstalled => {
      ctx.yard.mark_not_installable(&app_name, version)?;
      return Ok(None);
    }
  }
  // load again now that it is installed
  if let Some((executable, bin_folder)) = ctx.yard.load_executable(app_to_install.as_ref(), &executable_name, version, ctx) {
    let app_folder = ctx.yard.app_folder(&app_name, version);
    let args = executable_args.locate(&app_folder, &bin_folder)?;
    return Ok(Some(ExecutableCall { executable, args }));
  }
  Err(UserError::CannotFindExecutable)
}

// TODO: convert to named arguments
/// installs the given `NodeJS` package (if needed) and provides a call that executes it through `NodeJS`
fn load_or_install_nodejs_package(
  app_definition: &dyn AppDefinition,
  version: &Version,
  optional: bool,
  from_source: bool,
  ctx: &RuntimeContext,
  apps: &Apps,
) -> Result<Option<ExecutableCall>> {
  let app_name = app_definition.name();
  let app_folder = ctx.yard.app_folder(&app_name, version);
  // install the NodeJS package into its app folder if it isn't there yet
  if !app_folder.exists() {
    if ctx.yard.is_not_installable(&app_name, version) {
      return Ok(None);
    }
    match installation::any(app_definition, version, optional, from_source, ctx, apps)? {
      Outcome::Installed => {}
      Outcome::NotInstalled => {
        ctx.yard.mark_not_installable(&app_name, version)?;
        return Ok(None);
      }
    }
  }
  // app not installed --> check if uninstallable
  if ctx.yard.is_not_installable(&app_name, version) {
    return Ok(None);
  }
  // NodeJS will execute the package
  let node = NodeJS {};
  let node_versions = if let Some(versions) = ctx.config_file.lookup(&node.name()) {
    (*versions).clone()
  } else {
    RequestedVersions::from(node.latest_installable_version(ctx.log)?)
  };
  let Some(node_call) = load_or_install_app(&node, &node_versions, optional, false, ctx, apps)? else {
    return Ok(None);
  };
  // determine the main entry point for the npm package from the "bin" entry in the its package.json file
  let entry_point = load_entry_point(&app_folder, &app_name, version)?;
  let (executable, args) = node_call.with_args(vec![entry_point]);
  Ok(Some(ExecutableCall { executable, args }))
}

fn load_entry_point(app_folder: &Path, app_name: &ApplicationName, version: &Version) -> Result<String> {
  let package_src = app_folder.join("node_modules").join(app_name);
  let package_json_path = package_src.join("package.json");
  let content = fs::read_to_string(&package_json_path).map_err(|err| UserError::UnsupportedNpmPackage {
    app_name: app_name.clone(),
    version: version.clone(),
    err: format!("cannot find file {}: {}", package_json_path.display(), err),
  })?;
  let entry_point = parse_package_json(&content, app_name, version)?;
  Ok(package_src.join(entry_point).to_string_lossy().to_string())
}

fn parse_package_json(content: &str, app_name: &ApplicationName, version: &Version) -> Result<String> {
  let package_json: serde_json::Value = serde_json::from_str(content).map_err(|err| UserError::UnsupportedNpmPackage {
    app_name: app_name.clone(),
    version: version.clone(),
    err: format!("cannot parse package.json: {err}"),
  })?;
  match &package_json["bin"] {
    serde_json::Value::String(value) => Ok(value.clone()),
    serde_json::Value::Object(map) => {
      // prefer the entry whose key matches the app name
      if let Some(val) = map.get(app_name.as_str())
        && let Some(s) = val.as_str()
      {
        return Ok(s.to_string());
      }
      // if all values point to the same file, use that
      let files: AHashSet<&str> = map.values().filter_map(|v| v.as_str()).collect();
      if files.len() == 1 {
        #[allow(clippy::unwrap_used)]
        return Ok(files.into_iter().next().unwrap().to_string());
      }
      Err(UserError::UnsupportedNpmPackage {
        app_name: app_name.clone(),
        version: version.clone(),
        err: "cannot determine the entry point of the package".to_string(),
      })
    }
    serde_json::Value::Null => Err(UserError::UnsupportedNpmPackage {
      app_name: app_name.clone(),
      version: version.clone(),
      err: "package.json has no 'bin' entry".into(),
    }),
    _ => Err(UserError::UnsupportedNpmPackage {
      app_name: app_name.clone(),
      version: version.clone(),
      err: "package.json has an unknown 'bin' entry format".into(),
    }),
  }
}

#[cfg(test)]
mod tests {

  mod parse_package_json {
    use crate::applications::ApplicationName;
    use crate::commands::run::parse_package_json;
    use crate::{UserError, Version};
    use big_s::S;

    #[test]
    fn single_entry() {
      let app_name = ApplicationName::from("my-app");
      let version = Version::from("1.0.0");
      let content = r#"
{
  "name": "my-app",
  "bin": "index.js",
  "desc": "foo"
}"#;
      let result = parse_package_json(content, &app_name, &version);
      assert_eq!(result, Ok(S("index.js")));
    }

    #[test]
    fn multiple_entries_one_matches_name() {
      let app_name = ApplicationName::from("my-app");
      let version = Version::from("1.0.0");
      let content = r#"
{
  "name": "my-app",
  "bin": {
    "other": "other.js",
    "my-app": "my-app.js",
    "another-app": "another.js"
  },
  "desc": "foo"
}"#;
      let result = parse_package_json(content, &app_name, &version);
      assert_eq!(result, Ok(S("my-app.js")));
    }

    #[test]
    fn multiple_nonmatching_entries_all_point_to_the_same_file() {
      let app_name = ApplicationName::from("my-app");
      let version = Version::from("1.0.0");
      let content = r#"
{
  "name": "my-app",
  "bin": {
    "one": "my-app.js",
    "two": "my-app.js",
    "three": "my-app.js"
  },
  "desc": "foo"
}"#;
      let result = parse_package_json(content, &app_name, &version);
      assert_eq!(result, Ok(S("my-app.js")));
    }

    #[test]
    fn multiple_nonmatching_entries() {
      let app_name = ApplicationName::from("my-app");
      let version = Version::from("1.0.0");
      let content = r#"
{
  "name": "my-app",
  "bin": {
    "one": "one.js",
    "two": "two.js"
  },
  "desc": "foo"
}"#;
      let result = parse_package_json(content, &app_name, &version);
      assert_eq!(
        result,
        Err(UserError::UnsupportedNpmPackage {
          app_name,
          version,
          err: "cannot determine the entry point of the package".into(),
        })
      );
    }

    #[test]
    fn no_bin_entry() {
      let app_name = ApplicationName::from("my-app");
      let version = Version::from("1.0.0");
      let content = r#"
{
  "name": "my-app",
  "desc": "foo"
}"#;
      let result = parse_package_json(content, &app_name, &version);
      assert_eq!(
        result,
        Err(UserError::UnsupportedNpmPackage {
          app_name,
          version,
          err: "package.json has no 'bin' entry".into(),
        })
      );
    }

    #[test]
    fn empty_bin_entry() {
      let app_name = ApplicationName::from("my-app");
      let version = Version::from("1.0.0");
      let content = r#"
{
  "name": "my-app",
  "bin": {},
  "desc": "foo"
}"#;
      let result = parse_package_json(content, &app_name, &version);
      assert_eq!(
        result,
        Err(UserError::UnsupportedNpmPackage {
          app_name,
          version,
          err: "cannot determine the entry point of the package".into(),
        })
      );
    }
  }
}
