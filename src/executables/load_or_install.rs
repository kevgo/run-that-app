use crate::applications::{AppDefinition, ApplicationName, Apps, Npm};
use crate::configuration::{RequestedVersion, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::executables::{Executable, ExecutableArgs, ExecutableCall, ExecutableNameUnix, LoadAppOutcome, RunMethod, load_app_versions};
use crate::installation::Outcome;
use crate::yard::Yard;
use crate::{Version, installation};
use ahash::AHashSet;
use std::fs;
use std::path::Path;

pub fn load_or_install_apps(apps_to_include: Vec<&dyn AppDefinition>, apps: &Apps, optional: bool, ctx: &RuntimeContext) -> Result<Vec<ExecutableCall>> {
  let mut result = Vec::with_capacity(apps_to_include.len());
  for app_to_include in apps_to_include {
    match load_or_install_app_and_carrier(&LoadOrInstallAppAndCarrierArgs {
      app: app_to_include,
      cli_version: None,
      optional,
      from_source: false,
      ctx,
      apps,
    })? {
      LoadOrInstallAppOutcome::Loaded { executable_call } => result.push(executable_call),
      LoadOrInstallAppOutcome::NotInstallable { app: _ } => {}
    }
  }
  Ok(result)
}

/// Provides a callable that executes the given app
/// at the given CLI version if given,
/// otherwise the version in the given config file.
///
/// Also installs and uses the carrier app if one is needed.
pub fn load_or_install_app_and_carrier(args: &LoadOrInstallAppAndCarrierArgs) -> Result<LoadOrInstallAppOutcome> {
  match args.app.run_method(&Version::from("*"), args.ctx.platform) {
    RunMethod::ThisApp { install_methods: _ } => {
      // ignore the install methods here
      // - we loaded them with a fake version so they are not accurate
      // - we just need to know whether this app runs by itself or via a carrier here
      load_or_install_app(LoadOrInstallAppArgs {
        app: args.app,
        cli_version: args.cli_version,
        executable: args.app.executable_filename(),
        args: &ExecutableArgs::None,
        optional: args.optional,
        from_source: args.from_source,
        ctx: args.ctx,
        apps: args.apps,
      })
    }

    RunMethod::OtherAppOtherExecutable {
      app_definition: carrier_app,
      executable_name: carrier_executable,
    } => load_or_install_app(LoadOrInstallAppArgs {
      app: carrier_app.as_ref(),
      cli_version: args.cli_version,
      executable: carrier_executable,
      args: &ExecutableArgs::None,
      optional: args.optional,
      from_source: args.from_source,
      ctx: args.ctx,
      apps: args.apps,
    }),

    RunMethod::OtherAppDefaultExecutable {
      app_definition: carrier_app,
      args: carrier_args,
    } => load_or_install_app(LoadOrInstallAppArgs {
      app: carrier_app.as_ref(),
      cli_version: args.cli_version,
      executable: carrier_app.executable_filename(),
      args: &carrier_args,
      optional: args.optional,
      from_source: args.from_source,
      ctx: args.ctx,
      apps: args.apps,
    }),

    RunMethod::NodeJS { package } => {
      // step 1: ensure NodeJS is installed, install if needed
      let npm = Npm {};
      match load_or_install_app_and_carrier(&LoadOrInstallAppAndCarrierArgs {
        app: &npm,
        cli_version: None,
        optional: args.optional,
        from_source: false,
        ctx: args.ctx,
        apps: args.apps,
      })? {
        LoadOrInstallAppOutcome::Loaded { executable_call } => executable_call,
        LoadOrInstallAppOutcome::NotInstallable { app } => {
          println!("ERROR: cannot install NodeJS: {app}");
          return Ok(LoadOrInstallAppOutcome::NotInstallable { app });
        }
      };

      // step 2: determine the version of the npm package to run
      let app_versions = if let Some(version) = args.cli_version {
        RequestedVersions::from(version)
      } else if let Some(versions) = args.ctx.config_file.lookup(&args.app.name()) {
        versions.clone()
      } else {
        return Err(UserError::NoVersionsFound { app: args.app.name().clone() });
      };

      // step 3: fast-path: load the app executable
      match load_npm_entry_point_versions(args.app, package, &app_versions, args.ctx.yard)? {
        LoadAppOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppOutcome::Loaded { executable_call }),
        LoadAppOutcome::NotInstallable { app } => return Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
        LoadAppOutcome::NotInstalled { app: _ } => {}
      }

      // step 4: install the npm package
      match installation::versions(args.app, &app_versions, args.optional, args.from_source, args.ctx, args.apps)? {
        Outcome::Installed => {}
        Outcome::NotInstalled { app } => return Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
      }

      // step 5: load the npm package executable
      match load_npm_entry_point_versions(args.app, package, &app_versions, args.ctx.yard)? {
        LoadAppOutcome::Loaded { executable_call } => Ok(LoadOrInstallAppOutcome::Loaded { executable_call }),
        LoadAppOutcome::NotInstallable { app } => Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
        LoadAppOutcome::NotInstalled { app } => {
          println!("ERROR: this shouldn't happen, we just successfully installed {app} and now we can't load it");
          Ok(LoadOrInstallAppOutcome::NotInstallable { app })
        }
      }
    }
  }
}

pub struct LoadOrInstallAppAndCarrierArgs<'a> {
  pub app: &'a dyn AppDefinition,
  pub cli_version: Option<&'a Version>,
  pub optional: bool,
  pub from_source: bool,
  pub ctx: &'a RuntimeContext<'a>,
  pub apps: &'a Apps,
}

pub enum LoadOrInstallAppOutcome {
  Loaded { executable_call: ExecutableCall },
  NotInstallable { app: ApplicationName },
}

fn load_or_install_app(args: LoadOrInstallAppArgs) -> Result<LoadOrInstallAppOutcome> {
  // step 1: determine the version of the carrier app to install
  let carrier_versions = if let Some(version) = args.cli_version {
    RequestedVersions::from(version)
  } else if let Some(versions) = args.ctx.config_file.lookup(&args.app.name()) {
    versions.clone()
  } else {
    return Err(UserError::NoVersionsFound { app: args.app.name().clone() });
  };
  // step 2: fast-path: try to load the given carrier executable
  let executable = args.executable.platform_path(args.ctx.platform.os);
  match load_app_versions(args.app, &carrier_versions, &executable, args.args, args.ctx)? {
    LoadAppOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppOutcome::Loaded { executable_call }),
    LoadAppOutcome::NotInstallable { app } => return Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
    LoadAppOutcome::NotInstalled { app: _ } => {} // we'll install the app in the next step
  }
  // step 3: here the app needs to be installed --> install any of the configured versions
  match installation::versions(args.app, &carrier_versions, args.optional, args.from_source, args.ctx, args.apps)? {
    Outcome::Installed => {} // we'll load the app in the next step
    Outcome::NotInstalled { app } => {
      return Ok(LoadOrInstallAppOutcome::NotInstallable { app });
    }
  }
  // step 4: load the `carrier_executable_name` from the carrier directory
  match load_app_versions(args.app, &carrier_versions, &executable, args.args, args.ctx)? {
    LoadAppOutcome::Loaded { executable_call } => Ok(LoadOrInstallAppOutcome::Loaded { executable_call }),
    LoadAppOutcome::NotInstallable { app } => Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
    LoadAppOutcome::NotInstalled { app } => {
      println!("ERROR: this shouldn't happen, we just successfully installed {app} and now we can't load it");
      Ok(LoadOrInstallAppOutcome::NotInstallable { app })
    }
  }
}

struct LoadOrInstallAppArgs<'a> {
  app: &'a dyn AppDefinition,
  cli_version: Option<&'a Version>,
  executable: ExecutableNameUnix,
  args: &'a ExecutableArgs,
  optional: bool,
  from_source: bool,
  ctx: &'a RuntimeContext<'a>,
  apps: &'a Apps,
}

fn load_npm_entry_point_versions(app: &dyn AppDefinition, npm_package: &str, versions: &RequestedVersions, yard: &Yard) -> Result<LoadAppOutcome> {
  for version in versions {
    match version {
      RequestedVersion::Yard(version) => match load_npm_entry_point_version(app, npm_package, version, yard)? {
        LoadAppOutcome::Loaded { executable_call } => {
          return Ok(LoadAppOutcome::Loaded { executable_call });
        }
        LoadAppOutcome::NotInstalled { app } => {
          return Ok(LoadAppOutcome::NotInstalled { app });
        }
        LoadAppOutcome::NotInstallable { app: _ } => {}
      },
      RequestedVersion::Path(_version) => println!("ERROR: cannot load an npm entry point in the global path"),
    }
  }
  Ok(LoadAppOutcome::NotInstallable { app: app.name() })
}

fn load_npm_entry_point_version(app: &dyn AppDefinition, npm_package: &str, version: &Version, yard: &Yard) -> Result<LoadAppOutcome> {
  let app_name = app.name();
  let package_src = yard.app_folder(&app_name, version).join("node_modules").join(npm_package);
  let package_json_path = package_src.join("package.json");
  let Ok(content) = fs::read_to_string(&package_json_path) else {
    return Ok(LoadAppOutcome::NotInstalled { app: app_name });
  };
  let entry_point = parse_package_json(&content, &app_name, version, &package_json_path)?;
  let executable = package_src.join(entry_point);
  Ok(LoadAppOutcome::Loaded {
    executable_call: ExecutableCall {
      executable: Executable::from(executable),
      args: vec![],
    },
  })
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
