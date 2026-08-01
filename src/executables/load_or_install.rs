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

pub fn load_or_install_apps(apps_to_include: Vec<&dyn AppDefinition>, apps: &Apps, optional: bool, ctx: &RuntimeContext) -> Result<Vec<Executable>> {
  let mut result = Vec::with_capacity(apps_to_include.len());
  for app_to_include in apps_to_include {
    match load_or_install_app_and_carrier(LoadOrInstallAppAndCarrierArgs {
      app: app_to_include,
      cli_version: None,
      optional,
      from_source: false,
      ctx,
      apps,
    })? {
      LoadOrInstallAppOutcome::Loaded { executable } => result.push(executable),
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
      // step 1: ensure the carrier app is installed, install if needed
      if let Err(_err) = load_or_install_app_and_carrier(LoadOrInstallAppAndCarrierArgs {
        app: carrier.as_ref(),
        cli_version: None,
        optional,
        from_source: false,
        ctx,
        apps,
      }) {
        return Ok(LoadOrInstallAppOutcome::NotInstallable { app: carrier.name() });
      }
      // step 2: locate the shell script inside the carrier app
      let shell_script = locate_shell_script(carrier.as_ref(), cli_version, script_name, ctx)?;
      Ok(LoadOrInstallAppOutcome::Loaded { executable: shell_script })
    }

    RunMethod::NodeJS { package, script } => {
      // step 1: ensure NodeJS is installed, install if needed
      load_or_install_app_and_carrier(LoadOrInstallAppAndCarrierArgs {
        app: &NodeJS {},
        cli_version: None,
        optional,
        from_source: false,
        ctx,
        apps,
      })?;
      // step 2: determine the version of the npm package to run
      let app_versions = if let Some(version) = cli_version {
        RequestedVersions::from(version)
      } else if let Some(versions) = ctx.config_file.lookup(&app.name()) {
        versions.clone()
      } else {
        return Err(UserError::NoVersionsFound { app: app.name() });
      };
      // step 3: fast-path: try to load the app executable
      if let Ok(shell_script_path) = locate_npm_package_executable(app, &app_versions, script, ctx) {
        return Ok(LoadOrInstallAppOutcome::Loaded {
          executable: Executable::ShellScript(shell_script_path),
        });
      }
      // step 4: install the npm package
      match installation::versions(app, &app_versions, optional, from_source, ctx, apps)? {
        Outcome::Installed => {}
        Outcome::NotInstalled { app } => return Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
      }
      // step 5: load the npm package executable
      if let Ok(shell_script_path) = locate_npm_package_executable(app, &app_versions, script, ctx) {
        return Ok(LoadOrInstallAppOutcome::Loaded {
          executable: Executable::ShellScript(shell_script_path),
        });
      }
      println!("ERROR: this shouldn't happen, we just successfully installed npm package {package} and now we can't load it");
      Ok(LoadOrInstallAppOutcome::NotInstallable { app: package.into() })
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
  Loaded { executable: Executable },
  NotInstallable { app: ApplicationName },
}

fn locate_npm_package_executable(app: &dyn AppDefinition, versions: &RequestedVersions, script: &str, ctx: &RuntimeContext) -> Result<PathBuf> {
  let mut tried_paths = Vec::new();
  for version in versions {
    match version {
      RequestedVersion::Path(_version) => {
        (ctx.log)(Event::GlobalInstallSearch { binary: script });
        if let Ok(path) = which::which(script) {
          (ctx.log)(Event::GlobalInstallFound { path: &path });
          // TODO: verify the version here
          match app.analyze_executable(path)? {
            AnalyzeResult::NotIdentified { output } => todo!(),
            AnalyzeResult::IdentifiedButUnknownVersion => todo!(),
            AnalyzeResult::IdentifiedWithVersion(version) => todo!(),
          }
          return Ok(path);
        }
        (ctx.log)(Event::GlobalInstallNotFound);
        tried_paths.push(S("(global install)"));
      }
      RequestedVersion::Yard(version) => {
        let app_folder = ctx.yard.app_folder(&app.name(), version);
        let platform_script_name = script_name(script);
        let script_path = app_folder.join("node_modules").join(".bin").join(platform_script_name);
        if script_path.exists() {
          return Ok(script_path);
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

#[allow(clippy::panic)]
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
      RequestedVersion::Path(_version) => {
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
              panic!(
                "App {package} is an npm package, we should have handled this separately.\nPlease report this as a bug at https://github.com/kevgo/run-that-app"
              )
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
    LoadAppOutcome::Loaded { executable } => return Ok(LoadOrInstallAppOutcome::Loaded { executable }),
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
    LoadAppOutcome::Loaded { executable } => Ok(LoadOrInstallAppOutcome::Loaded { executable }),
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
  executable_name: ExecutableNameUnix,
  optional: bool,
  from_source: bool,
  ctx: &'a RuntimeContext<'a>,
  apps: &'a Apps,
}
