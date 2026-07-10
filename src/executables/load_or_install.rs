use crate::applications::{AnalyzeResult, AppDefinition, ApplicationName, Apps, NodeJS, Npm, carrier};
use crate::configuration::{self, AppVersions, RequestedVersion, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::executables::{ExecutableArgs, ExecutableCall, ExecutableCallDefinition, LoadAppVersionsOutcome, RunMethod, load_app_versions};
use crate::filesystem::find_global_install;
use crate::installation::Outcome;
use crate::logging::Event;
use crate::{GetCmdArgs, Version, get_cmd, installation};
use ahash::AHashSet;
use std::fs;
use std::path::Path;

// pub fn load_or_install_apps(
//   apps_versions: &Vec<AppVersions>,
//   apps: &Apps,
//   optional: bool,
//   from_source: bool,
//   ctx: &RuntimeContext,
// ) -> Result<Vec<ExecutableCall>> {
//   let mut result = vec![];
//   for app_versions in apps_versions {
//     let app = apps.lookup(&app_versions.app_name)?;
//     if let Some(executable_call) = load_or_install_app_with_carrier(app, &app_versions.versions, optional, from_source, ctx, apps)? {
//       result.push(executable_call);
//     }
//   }
//   Ok(result)
// }

/// Provides a callable that executes the given app
/// at the given CLI version if given,
/// otherwise the version in the given config file.
///
/// Installs and uses the carrier app if one is needed.
pub fn load_or_install_app_and_carrier(
  app_definition: &dyn AppDefinition,
  cli_version: Option<&Version>,
  config_file: &configuration::File,
  optional: bool,
  from_source: bool,
  ctx: &RuntimeContext,
  apps: &Apps,
) -> Result<LoadOrInstallAppWithCarrierOutcome> {
  match app_definition.run_method(&Version::from("*"), ctx.platform) {
    RunMethod::ThisApp { install_methods: _ } => {
      // ignore the install methods here
      // - we loaded them with a fake version so they are not accurate
      // - we just need to know whether this app runs by itself or via a carrier
      //
      // step 1: determine the possible versions of the app
      let versions = if let Some(version) = cli_version {
        RequestedVersions::from(version)
      } else if let Some(versions) = config_file.lookup(&app_definition.name()) {
        versions.clone()
      } else {
        return Err(UserError::NoVersionsFound {
          app: app_definition.name().clone(),
        });
      };
      // step 2: fast-path: try to load the app
      let executable = app_definition.executable_filename().platform_path(ctx.platform.os);
      match load_app_versions(app_definition, &versions, &executable, ExecutableArgs::None, ctx)? {
        LoadAppVersionsOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call }),
        LoadAppVersionsOutcome::NotInstallable => return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable),
        LoadAppVersionsOutcome::NotInstalled => {}
      };
      // step 3: slow-path: here the app needs to be installed --> install any of the configured versions
      match installation::versions(app_definition, &versions, optional, from_source, ctx, apps)? {
        Outcome::Installed => {}
        Outcome::NotInstalled => {
          return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable);
        }
      }
      // step 4: load the now installed app
      match load_app_versions(app_definition, &versions, &executable, ExecutableArgs::None, ctx)? {
        LoadAppVersionsOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call }),
        LoadAppVersionsOutcome::NotInstallable => return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable),
        LoadAppVersionsOutcome::NotInstalled => {
          panic!("this shouldn't really happen, we just successfully installed the app and now we can't load it")
        }
      };
    }

    RunMethod::OtherAppOtherExecutable {
      app_definition: carrier_app,
      executable_name: carrier_executable_name,
    } => {
      // step 1: determine the version of the carrier app to install
      let carrier_versions = if let Some(version) = cli_version {
        RequestedVersions::from(version)
      } else if let Some(versions) = ctx.config_file.lookup(&carrier_app.name()) {
        versions.clone()
      } else {
        return Err(UserError::NoVersionsFound {
          app: carrier_app.name().clone(),
        });
      };
      // step 2: fast-path: try to load the given carrier executable
      let carrier_executable = carrier_executable_name.platform_path(ctx.platform.os);
      match load_app_versions(carrier_app.as_ref(), &carrier_versions, &carrier_executable, ExecutableArgs::None, ctx)? {
        LoadAppVersionsOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call }),
        LoadAppVersionsOutcome::NotInstallable => return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable),
        LoadAppVersionsOutcome::NotInstalled => {}
      };
      // step 3: slow-path: here the app needs to be installed --> install any of the configured versions
      match installation::versions(app_definition, &carrier_versions, optional, from_source, ctx, apps)? {
        Outcome::Installed => {}
        Outcome::NotInstalled => {
          return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable);
        }
      }
      // step 4: load the `carrier_executable_name` from the carrier directory
      match load_app_versions(app_definition, &carrier_versions, &carrier_executable, ExecutableArgs::None, ctx)? {
        LoadAppVersionsOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call }),
        LoadAppVersionsOutcome::NotInstallable => return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable),
        LoadAppVersionsOutcome::NotInstalled => {
          panic!("this shouldn't really happen, we just successfully installed the app and now we can't load it")
        }
      };
    }

    RunMethod::OtherAppDefaultExecutable {
      app_definition: carrier_app,
      args: carrier_args,
    } => {
      // step 1: determine the version of the carrier app to install
      let carrier_versions = if let Some(version) = cli_version {
        RequestedVersions::from(version)
      } else if let Some(versions) = ctx.config_file.lookup(&carrier_app.name()) {
        versions.clone()
      } else {
        return Err(UserError::NoVersionsFound {
          app: carrier_app.name().clone(),
        });
      };
      // step 2: fast-path: try to load the given carrier executable
      let carrier_executable = carrier_app.executable_filename().platform_path(ctx.platform.os);
      match load_app_versions(carrier_app.as_ref(), &carrier_versions, &carrier_executable, carrier_args.clone(), ctx)? {
        LoadAppVersionsOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call }),
        LoadAppVersionsOutcome::NotInstallable => return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable),
        LoadAppVersionsOutcome::NotInstalled => {}
      };
      // step 3: slow-path: here the app needs to be installed --> install any of the configured versions
      match installation::versions(app_definition, &carrier_versions, optional, from_source, ctx, apps)? {
        Outcome::Installed => {}
        Outcome::NotInstalled => {
          return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable);
        }
      }
      // step 4: load the `carrier_executable_name` from the carrier directory
      match load_app_versions(app_definition, &carrier_versions, &carrier_executable, carrier_args, ctx)? {
        LoadAppVersionsOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call }),
        LoadAppVersionsOutcome::NotInstallable => return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable),
        LoadAppVersionsOutcome::NotInstalled => {
          panic!("this shouldn't really happen, we just successfully installed the app and now we can't load it")
        }
      };
    }

    RunMethod::NodeJS { package } => {
      // step 1: determine the version of the npm package to run
      let app_versions = if let Some(version) = cli_version {
        RequestedVersions::from(version)
      } else if let Some(versions) = ctx.config_file.lookup(&app_definition.name()) {
        versions.clone()
      } else {
        return Err(UserError::NoVersionsFound {
          app: app_definition.name().clone(),
        });
      };

      // step 2: fast-path: load the npm package executable

      // step 2: determine the version of Node to install
      let node = NodeJS {};
      let Some(node_versions) = ctx.config_file.lookup(&node.name()) else {
        return Err(UserError::NoVersionsFound { app: node.name().clone() });
      };
      // step 2: fast-path: try to load npm (and thereby check that node is installed)
      let npm = Npm {};
      let get_npm_args = GetCmdArgs {
        version: None,
        app_args: vec!["--version".into()],
        from_source: false,
        include_apps: vec![],
        optional: false,
        verbose: false,
      };
      let npm_cmd = get_cmd(&npm, get_npm_args, apps)?;
      match load_app_versions(carrier_app.as_ref(), &carrier_versions, &carrier_executable, carrier_args.clone(), ctx)? {
        LoadAppVersionsOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call }),
        LoadAppVersionsOutcome::NotInstallable => return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable),
        LoadAppVersionsOutcome::NotInstalled => {}
      };

      // step 3: slow-path: install node

      // step 4: load the npm executable

      // step 5: fast-path: try to load the npm package executable

      // step 6: slow-path: install the npm package

      // step 7: load the npm package executable

      // step 2: fast-path: try to load the given carrier executable
      let carrier_executable = carrier_executable_name.platform_path(ctx.platform.os);
      match load_app_versions(carrier_app.as_ref(), &carrier_versions, &carrier_executable, ExecutableArgs::None, ctx)? {
        LoadAppVersionsOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call }),
        LoadAppVersionsOutcome::NotInstallable => return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable),
        LoadAppVersionsOutcome::NotInstalled => {}
      };
      // step 3: slow-path: here the app needs to be installed --> install any of the configured versions
      match installation::versions(app_definition, &carrier_versions, optional, from_source, ctx, apps)? {
        Outcome::Installed => {}
        Outcome::NotInstalled => {
          return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable);
        }
      }
      // step 4: load the `carrier_executable_name` from the carrier directory
      match load_app_versions(app_definition, &carrier_versions, &carrier_executable, ExecutableArgs::None, ctx)? {
        LoadAppVersionsOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call }),
        LoadAppVersionsOutcome::NotInstallable => return Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable),
        LoadAppVersionsOutcome::NotInstalled => {
          panic!("this shouldn't really happen, we just successfully installed the app and now we can't load it")
        }
      };
      // step 2: install (not load) Node at that version if needed
      // step 3: create an ExecutableCall that runs "npm" from the node directory
      // step 4: install the package by running "npm install <package>" in the package directory
      // step 5: return a callable that runs the npm package's executable in "node_modules/.bin/<package>"
    }
  }
  Ok(LoadOrInstallAppWithCarrierOutcome::NotInstallable)
}

enum LoadOrInstallAppWithCarrierOutcome {
  Loaded { executable_call: ExecutableCall },
  NotInstallable,
}

pub fn load_or_install_app_versions(
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

/// Loads or installs the given app at the given version
/// and returns a callable that executes it.
/// If the app runs via a carrier app,
/// installs the carrier app as well.
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
    RequestedVersion::Yard(version) => {
      // load or install the app
      ctx.yard.with_lock(&app_definition.name(), version, ctx, || {
        load_or_install_from_yard(app_definition, version, optional, from_source, ctx, apps)
      })
    }
  }
}

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
  if let RunMethod::NodeJS { package } = app_definition.run_method(version, ctx.platform) {
    return load_or_install_nodejs_package(app_definition, version, package, optional, from_source, ctx, apps);
  }
  let (app_to_install, executable_name, executable_args) = carrier(app_definition, version, ctx.platform);
  let app_name = app_to_install.name();
  // try to load the app
  if let Some((executable, bin_folder)) = ctx.yard.load_executable(app_to_install.as_ref(), &executable_name, version, ctx) {
    let app_folder = ctx.yard.app_folder(&app_name, version);
    let args = executable_args.locate(&app_name, version, &app_folder, &bin_folder)?;
    return Ok(Some(ExecutableCall { executable, args }));
  }
  // app not installed --> check if uninstallable
  if ctx.yard.is_not_installable(&app_name, version) {
    return Ok(None);
  }
  // app not installed and installable --> try to install
  match installation::version(app_to_install.as_ref(), version, optional, from_source, ctx, apps)? {
    Outcome::Installed => {} // we'll load it below
    Outcome::NotInstalled => {
      ctx.yard.mark_not_installable(&app_name, version)?;
      return Ok(None);
    }
  }
  // load again now that it is installed
  if let Some((executable, bin_folder)) = ctx.yard.load_executable(app_to_install.as_ref(), &executable_name, version, ctx) {
    let app_folder = ctx.yard.app_folder(&app_name, version);
    let args = executable_args.locate(&app_name, version, &app_folder, &bin_folder)?;
    return Ok(Some(ExecutableCall { executable, args }));
  }
  Err(UserError::CannotFindExecutable {
    app: app_name.clone(),
    version: version.clone(),
  })
}

/// installs the given `NodeJS` package (if needed) and provides a call that executes it through `NodeJS`
fn load_or_install_nodejs_package(
  app_definition: &dyn AppDefinition,
  version: &Version,
  npm_package: &str,
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
    match installation::version(app_definition, version, optional, from_source, ctx, apps)? {
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
  let entry_point = load_entry_point(&app_folder, &app_name, npm_package, version)?;
  let (executable, args) = node_call.with_args(vec![entry_point]);
  Ok(Some(ExecutableCall { executable, args }))
}

fn load_entry_point(app_folder: &Path, app_name: &ApplicationName, npm_package: &str, version: &Version) -> Result<String> {
  let package_src = app_folder.join("node_modules").join(npm_package);
  let package_json_path = package_src.join("package.json");
  let content = fs::read_to_string(&package_json_path).map_err(|err| UserError::UnsupportedNpmPackage {
    app_name: app_name.clone(),
    version: version.clone(),
    err: format!("cannot find file {}: {}", package_json_path.display(), err),
  })?;
  let entry_point = parse_package_json(&content, app_name, version, &package_json_path)?;
  Ok(package_src.join(entry_point).to_string_lossy().to_string())
}

fn parse_package_json(content: &str, app_name: &ApplicationName, version: &Version, package_json_path: &Path) -> Result<String> {
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
      err: format!("{} has no 'bin' entry", package_json_path.display()),
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
    use crate::executables::load_or_install::parse_package_json;
    use crate::{UserError, Version};
    use big_s::S;
    use std::path::Path;

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
      let result = parse_package_json(content, &app_name, &version, Path::new("package.json"));
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
      let result = parse_package_json(content, &app_name, &version, Path::new("package.json"));
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
      let result = parse_package_json(content, &app_name, &version, Path::new("package.json"));
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
      let result = parse_package_json(content, &app_name, &version, Path::new("package.json"));
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
      let result = parse_package_json(content, &app_name, &version, Path::new("package.json"));
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
      let result = parse_package_json(content, &app_name, &version, Path::new("package.json"));
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
