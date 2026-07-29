use crate::applications::{AppDefinition, ApplicationName, Apps, NodeJS};
use crate::configuration::{RequestedVersion, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::executables::{ExecutableCall, ExecutableNameUnix, LoadAppOutcome, RunMethod, load_app_versions};
use crate::installation::{BinFolder, Outcome};
use crate::logging::Event;
use crate::{Version, installation, subshell};
use big_s::S;
use std::path::PathBuf;

pub fn load_or_install_apps(
  apps_to_include: Vec<&dyn AppDefinition>,
  apps: &Apps,
  app_args: &[String],
  optional: bool,
  ctx: &RuntimeContext,
) -> Result<Vec<ExecutableCall>> {
  let mut result = Vec::with_capacity(apps_to_include.len());
  for app_to_include in apps_to_include {
    match load_or_install_app_and_carrier(LoadOrInstallAppAndCarrierArgs {
      app: app_to_include,
      cli_version: None,
      app_args,
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
pub fn load_or_install_app_and_carrier(
  LoadOrInstallAppAndCarrierArgs {
    app,
    cli_version,
    app_args,
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
        executable: app.executable_filename(),
        app_args,
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
      executable: carrier_executable,
      app_args,
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
        app_args: &[],
        optional,
        from_source: false,
        ctx,
        apps,
      }) {
        return Ok(LoadOrInstallAppOutcome::NotInstallable { app: carrier.name() });
      }
      // step 2: locate the shell script inside the carrier app
      let shell_script = locate_shell_script(carrier.as_ref(), cli_version, script_name, ctx)?;
      // step 3: create the executable call that runs the shell script
      let executable_call = subshell::shell_script_call(&shell_script, app_args);
      Ok(LoadOrInstallAppOutcome::Loaded { executable_call })
    }

    RunMethod::NodeJS { package, script } => {
      // step 1: ensure NodeJS is installed, install if needed
      load_or_install_app_and_carrier(LoadOrInstallAppAndCarrierArgs {
        app: &NodeJS {},
        cli_version: None,
        app_args: &[],
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
        return Err(UserError::NoVersionsFound { app: app.name().clone() });
      };
      // step 3: fast-path: load the app executable
      if let Ok(shell_script) = locate_shell_script(app, cli_version, script, ctx) {
        return Ok(LoadOrInstallAppOutcome::Loaded {
          executable_call: subshell::shell_script_call(&shell_script, app_args),
        });
      }
      // step 4: install the npm package
      match installation::versions(app, &app_versions, optional, from_source, ctx, apps)? {
        Outcome::Installed => {}
        Outcome::NotInstalled { app } => return Ok(LoadOrInstallAppOutcome::NotInstallable { app }),
      }
      // step 5: load the npm package executable
      if let Ok(shell_script) = locate_shell_script(app, cli_version, script, ctx) {
        return Ok(LoadOrInstallAppOutcome::Loaded {
          executable_call: subshell::shell_script_call(&shell_script, app_args),
        });
      }
      println!("ERROR: this shouldn't happen, we just successfully installed {package} and now we can't load it");
      Ok(LoadOrInstallAppOutcome::NotInstallable { app: package.into() })
    }
  }
}

pub struct LoadOrInstallAppAndCarrierArgs<'a> {
  pub app: &'a dyn AppDefinition,
  pub cli_version: Option<&'a Version>,
  pub app_args: &'a [String],
  pub optional: bool,
  pub from_source: bool,
  pub ctx: &'a RuntimeContext<'a>,
  pub apps: &'a Apps,
}

pub enum LoadOrInstallAppOutcome {
  Loaded { executable_call: ExecutableCall },
  NotInstallable { app: ApplicationName },
}

fn locate_shell_script(carrier: &dyn AppDefinition, cli_version: Option<&Version>, script_name: &str, ctx: &RuntimeContext) -> Result<PathBuf> {
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
        if let Ok(path) = which::which(script_name) {
          (ctx.log)(Event::GlobalInstallFound { path: &path });
          // Note: we cannot verify the version here because shell scripts usually get versioned together with their carrier app
          return Ok(path);
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
            | installation::Method::CompileRustRepo { url: _ }
            | installation::Method::InstallNodeJSPackage { package: _, script: _ } => bin_folders.push(BinFolder::Subfolder {
              path: PathBuf::from("node_modules").join(".bin"),
            }),
          }
        }
        let mut bin_folder_paths = Vec::new();
        for bin_folder in bin_folders {
          bin_folder_paths.extend(bin_folder.possible_paths(&app_folder));
        }
        for bin_folder in bin_folder_paths {
          let app_bin_folder = app_folder.join(&bin_folder);
          let path = app_bin_folder.join(script_name);
          (ctx.log)(Event::YardCheckExistingAppBegin { path: &path });
          if path.exists() {
            (ctx.log)(Event::YardCheckExistingAppFound);
            return Ok(path);
          }
          (ctx.log)(Event::YardCheckExistingAppNotFound);
          tried_paths.push(path.to_string_lossy().to_string());
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
    executable,
    app_args,
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
  let executable = executable.platform_path(ctx.platform.os);
  match load_app_versions(app, &versions, &executable, app_args, ctx)? {
    LoadAppOutcome::Loaded { executable_call } => return Ok(LoadOrInstallAppOutcome::Loaded { executable_call }),
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
  match load_app_versions(app, &versions, &executable, app_args, ctx)? {
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
  app_args: &'a [String],
  optional: bool,
  from_source: bool,
  ctx: &'a RuntimeContext<'a>,
  apps: &'a Apps,
}
