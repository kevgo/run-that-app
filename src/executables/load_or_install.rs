use crate::applications::{AnalyzeResult, AppDefinition, ApplicationName, Apps, NodeJS};
use crate::configuration::{RequestedVersion, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::executables::{Executable, ExecutableNameUnix, LoadAppOutcome, RunMethod, load_app_versions};
use crate::installation::Outcome;
use crate::logging::Event;
use crate::{Version, installation};
use big_s::S;
use std::path::PathBuf;

pub fn load_or_install_apps(
  apps: &Apps,
  optional: bool,
  apps_to_include: Vec<&dyn AppDefinition>,
  ctx: &RuntimeContext,
) -> Result<(Vec<Executable>, Vec<PathBuf>)> {
  let mut executables = Vec::with_capacity(apps_to_include.len());
  let mut extra_paths = Vec::new();
  for app_to_include in apps_to_include {
    match load_or_install_app_and_carrier(LoadOrInstallAppAndCarrierArgs {
      app: app_to_include,
      cli_version: None,
      optional,
      from_source: false,
      ctx,
      apps,
    })? {
      LoadOrInstallAppOutcome::Loaded { executable, extra_path } => {
        executables.push(executable);
        extra_paths.extend(extra_path);
      }
      LoadOrInstallAppOutcome::NotInstallable { app: _ } if optional => {}
      LoadOrInstallAppOutcome::NotInstallable { app } => return Err(UserError::UnsupportedPlatform { app }),
    }
  }
  Ok((executables, extra_paths))
}

/// Provides a callable that executes the given app
/// at the given CLI version if given,
/// otherwise the version in the given config file.
///
/// Also installs and uses the carrier app if one is needed.
pub fn load_or_install_app_and_carrier(
  LoadOrInstallAppAndCarrierArgs {
    app,
    cli_version,
    optional,
    from_source,
    ctx,
    apps,
  }: LoadOrInstallAppAndCarrierArgs,
) -> Result<LoadOrInstallAppOutcome> {
  match app.run_method(&Version::from("*"), ctx.platform) {
    RunMethod::ThisApp { install_methods: _ } => {
      // ignore the install methods here
      // - we loaded them with a fake version so they are not accurate
      // - we just need to know whether this app runs by itself or via a carrier here
      load_or_install_app(LoadOrInstallAppArgs {
        app,
        cli_version,
        executable_name: app.executable_filename(),
        optional,
        from_source,
        ctx,
        apps,
      })
    }

    RunMethod::OtherAppOtherExecutable {
      carrier,
      executable_name: carrier_executable,
    } => load_or_install_app(LoadOrInstallAppArgs {
      app: carrier.as_ref(),
      cli_version,
      executable_name: carrier_executable,
      optional,
      from_source,
      ctx,
      apps,
    }),

    RunMethod::OtherAppShellScript { carrier, script_name } => {
      // step 1: load the carrier app, install if needed
      let carrier_paths = match load_or_install_app_and_carrier(LoadOrInstallAppAndCarrierArgs {
        app: carrier.as_ref(),
        cli_version: None,
        optional,
        from_source: false,
        ctx,
        apps,
      })? {
        LoadOrInstallAppOutcome::Loaded {
          executable: carrier_exe,
          // The path of the carrier's carrier.
          // Probably a bit excessive to go that deep, but we have it so let's do the right thing.
          extra_path: carrier_carrier_path,
        } => {
          let mut carrier_paths = Vec::with_capacity(carrier_carrier_path.len() + 1);
          carrier_paths.extend(carrier_carrier_path);
          let carrier_path = carrier_exe.parent_path().to_path_buf();
          carrier_paths.push(carrier_path);
          carrier_paths
        }
        LoadOrInstallAppOutcome::NotInstallable { app } => {
          return Ok(LoadOrInstallAppOutcome::NotInstallable { app });
        }
      };
      // step 2: locate the shell script inside the carrier app
      let shell_script = locate_shell_script(carrier.as_ref(), cli_version, script_name, ctx)?;
      Ok(LoadOrInstallAppOutcome::Loaded {
        executable: shell_script,
        extra_path: carrier_paths,
      })
    }

    RunMethod::NodeJS { package, script } => {
      // step 1: load NodeJS, install if needed, and put it on PATH
      let node_paths = match load_or_install_app_and_carrier(LoadOrInstallAppAndCarrierArgs {
        app: &NodeJS {},
        cli_version: None,
        optional,
        from_source: false,
        ctx,
        apps,
      }) {
        Ok(LoadOrInstallAppOutcome::Loaded {
          executable: node,
          // the path of Node's carrier app
          extra_path: node_carrier_path,
        }) => {
          let mut carrier_paths = Vec::with_capacity(node_carrier_path.len() + 1);
          carrier_paths.extend(node_carrier_path);
          let node_path = node.parent_path().to_path_buf();
          carrier_paths.push(node_path);
          carrier_paths
        }
        Ok(LoadOrInstallAppOutcome::NotInstallable { app: _ }) if optional => {
          return Ok(LoadOrInstallAppOutcome::NotInstallable { app: app.name() });
        }
        Ok(LoadOrInstallAppOutcome::NotInstallable { app: node }) => return Err(UserError::UnsupportedPlatform { app: node }),
        Err(UserError::NoVersionsFound { app: runtime }) => {
          return Err({
            UserError::MissingRuntime {
              runtime,
              needed_by: app.name(),
              script: None,
              searched_dirs: vec![],
            }
          });
        }
        Err(err) => return Err(err),
      };
      // step 2: determine the version of the npm package to run
      let app_versions = if let Some(version) = cli_version {
        RequestedVersions::from(version)
      } else if let Some(versions) = ctx.config_file.lookup(&app.name()) {
        versions.clone()
      } else {
        return Err(UserError::NoVersionsFound { app: app.name() });
      };
      // step 3: fast-path: try to load the app executable
      if let Ok(executable) = locate_npm_package_executable(app, &app_versions, script, ctx) {
        return Ok(LoadOrInstallAppOutcome::Loaded {
          executable,
          extra_path: node_paths,
        });
      }
      // step 4: install the npm package
      match installation::versions(app, &app_versions, optional, from_source, ctx, apps)? {
        Outcome::Installed => {}
        Outcome::NotInstalled { app } => return Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
      }
      // step 5: load the npm package executable
      if let Ok(executable) = locate_npm_package_executable(app, &app_versions, script, ctx) {
        return Ok(LoadOrInstallAppOutcome::Loaded {
          executable,
          extra_path: node_paths,
        });
      }
      Err(UserError::InternalError {
        message: format!("successfully installed npm package {package} but cannot load it now"),
      })
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
  Loaded {
    executable: Executable,
    /// directories to prepend to PATH when running this executable,
    /// usually the carrier, e.g. Node.js for npm packages
    extra_path: Vec<PathBuf>,
  },
  NotInstallable {
    app: ApplicationName,
  },
}

fn locate_npm_package_executable(app: &dyn AppDefinition, versions: &RequestedVersions, script: &str, ctx: &RuntimeContext) -> Result<Executable> {
  let mut tried_paths = Vec::new();
  for version in versions {
    match version {
      RequestedVersion::Path(range) => {
        (ctx.log)(Event::GlobalInstallSearch { binary: script });
        if let Ok(path) = which::which(script) {
          (ctx.log)(Event::GlobalInstallFound { path: &path });
          let executable = Executable::ShellScript(path);
          match app.analyze_executable(&executable)? {
            AnalyzeResult::NotIdentified { output: _ } => {
              (ctx.log)(Event::GlobalInstallNotIdentified);
              continue;
            }
            AnalyzeResult::IdentifiedButUnknownVersion if range.to_string() == "*" => {
              (ctx.log)(Event::GlobalInstallMatchingVersion { range, version: None });
              return Ok(executable);
            }
            AnalyzeResult::IdentifiedButUnknownVersion => {
              (ctx.log)(Event::GlobalInstallMismatchingVersion { range, version: None });
              continue;
            }
            AnalyzeResult::IdentifiedWithVersion(version) if range.matches(&version.semver()?) => {
              (ctx.log)(Event::GlobalInstallMatchingVersion {
                range,
                version: Some(&version),
              });
              return Ok(executable);
            }
            AnalyzeResult::IdentifiedWithVersion(version) => {
              (ctx.log)(Event::GlobalInstallMatchingVersion {
                range,
                version: Some(&version),
              });
              continue;
            }
          }
        }
        (ctx.log)(Event::GlobalInstallNotFound);
        tried_paths.push(S("(global install)"));
      }
      RequestedVersion::Yard(version) => {
        let app_folder = ctx.yard.app_folder(&app.name(), version);
        let platform_script_name = script_name(script);
        let script_path = app_folder.join("node_modules").join(".bin").join(platform_script_name);
        if script_path.exists() {
          return Ok(Executable::ShellScript(script_path));
        }
        tried_paths.push(script_path.to_string_lossy().to_string());
      }
    }
  }
  Err(UserError::CannotFindScript {
    name: script.to_string(),
    paths: tried_paths,
  })
}

#[cfg(not(windows))]
fn script_name(unix_script_name: &str) -> String {
  unix_script_name.to_string()
}

#[cfg(windows)]
fn script_name(unix_script_name: &str) -> String {
  format!("{unix_script_name}.cmd")
}

fn locate_shell_script(carrier: &dyn AppDefinition, cli_version: Option<&Version>, script_name: &str, ctx: &RuntimeContext) -> Result<Executable> {
  // step 1: determine the version of the app to install
  let versions = if let Some(version) = cli_version {
    RequestedVersions::from(version)
  } else if let Some(versions) = ctx.config_file.lookup(&carrier.name()) {
    versions.clone()
  } else {
    return Err(UserError::NoVersionsFound { app: carrier.name() });
  };
  // step 2: find the first matching candidate
  let mut tried_paths = Vec::new();
  for version in &versions {
    match version {
      RequestedVersion::Path(_range) => {
        (ctx.log)(Event::GlobalInstallSearch { binary: script_name });
        if let Ok(script_path) = which::which(script_name) {
          (ctx.log)(Event::GlobalInstallFound { path: &script_path });
          // Note: we cannot verify the version here because shell scripts usually get versioned together with their carrier app
          return Ok(Executable::ShellScript(script_path));
        }
        (ctx.log)(Event::GlobalInstallNotFound);
        tried_paths.push(S("(global install)"));
      }
      RequestedVersion::Yard(version) => {
        let app_folder = ctx.yard.app_folder(&carrier.name(), version);
        // find the bin folders
        let install_methods = match carrier.run_method(version, ctx.platform) {
          RunMethod::ThisApp { install_methods } => install_methods,
          RunMethod::OtherAppOtherExecutable {
            carrier: _,
            executable_name: _,
          }
          | RunMethod::OtherAppShellScript { carrier: _, script_name: _ }
          | RunMethod::NodeJS { package: _, script: _ } => vec![],
        };
        let mut bin_folders = Vec::new();
        for install_method in install_methods {
          match install_method {
            installation::Method::DownloadArchive { url: _, bin_folder } | installation::Method::CompileRustCrate { name: _, bin_folder } => {
              bin_folders.push(bin_folder);
            }
            installation::Method::DownloadExecutable { url: _ }
            | installation::Method::CompileGoSource { import_path: _ }
            | installation::Method::CompileRustRepo { url: _ } => {}
            installation::Method::InstallNodeJSPackage { package, script: _ } => {
              return Err(UserError::InternalError {
                message: format!(
                  "App {package} is an npm package, we should have handled this separately.\nPlease report this as a bug at https://github.com/kevgo/run-that-app"
                ),
              });
            }
          }
        }
        let mut bin_folder_paths = Vec::new();
        for bin_folder in bin_folders {
          bin_folder_paths.extend(bin_folder.possible_paths(&app_folder));
        }
        for bin_folder in bin_folder_paths {
          let app_bin_folder = app_folder.join(&bin_folder);
          let script_path = app_bin_folder.join(script_name);
          (ctx.log)(Event::YardCheckExistingAppBegin { path: &script_path });
          if script_path.exists() {
            (ctx.log)(Event::YardCheckExistingAppFound);
            return Ok(Executable::ShellScript(script_path));
          }
          (ctx.log)(Event::YardCheckExistingAppNotFound);
          tried_paths.push(script_path.to_string_lossy().to_string());
        }
      }
    }
  }
  Err(UserError::CannotFindScript {
    name: script_name.to_string(),
    paths: tried_paths,
  })
}

/// Loads or installs only the given app (not its carrier) and returns the executable call.
fn load_or_install_app(
  LoadOrInstallAppArgs {
    app,
    cli_version,
    executable_name,
    optional,
    from_source,
    ctx,
    apps,
  }: LoadOrInstallAppArgs,
) -> Result<LoadOrInstallAppOutcome> {
  // step 1: determine the version of the app to install
  let versions = if let Some(version) = cli_version {
    RequestedVersions::from(version)
  } else if let Some(versions) = ctx.config_file.lookup(&app.name()) {
    versions.clone()
  } else {
    return Err(UserError::NoVersionsFound { app: app.name() });
  };
  // step 2: fast-path: try to load the given executable for the given app
  let executable = executable_name.platform_path(ctx.platform.os);
  match load_app_versions(app, &versions, &executable, ctx)? {
    LoadAppOutcome::Loaded { executable } => {
      return Ok(LoadOrInstallAppOutcome::Loaded {
        executable,
        extra_path: vec![],
      });
    }
    LoadAppOutcome::NotInstallable { app } => return Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
    LoadAppOutcome::NotInstalled { app: _ } => {} // we'll install the app in the next step
  }
  // step 3: here the app needs to be installed --> install any of its given versions
  match installation::versions(app, &versions, optional, from_source, ctx, apps)? {
    Outcome::Installed => {} // we'll load the app in the next step
    Outcome::NotInstalled { app } => {
      return Ok(LoadOrInstallAppOutcome::NotInstallable { app });
    }
  }
  // step 4: load the executable for the given app
  match load_app_versions(app, &versions, &executable, ctx)? {
    LoadAppOutcome::Loaded { executable } => Ok(LoadOrInstallAppOutcome::Loaded {
      executable,
      extra_path: vec![],
    }),
    LoadAppOutcome::NotInstallable { app } => Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
    LoadAppOutcome::NotInstalled { app } => Err(UserError::InternalError {
      message: format!("successfully installed {app} but cannot load it now"),
    }),
  }
}

struct LoadOrInstallAppArgs<'a> {
  app: &'a dyn AppDefinition,
  cli_version: Option<&'a Version>,
  executable_name: ExecutableNameUnix,
  optional: bool,
  from_source: bool,
  ctx: &'a RuntimeContext<'a>,
  apps: &'a Apps,
}
