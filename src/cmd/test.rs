use crate::apps::AnalyzeResult;
use crate::config::AppName;
use crate::install;
use crate::logger::Event;
use crate::subshell::Executable;
use crate::{apps, logger, platform, yard, Result};
use colored::Colorize;
use std::io;
use std::process::ExitCode;

pub fn test(mut start_at_app: Option<AppName>, verbose: bool) -> Result<ExitCode> {
    let apps = apps::all();
    let log = logger::new(verbose);
    let platform = platform::detect(log)?;
    let temp_folder = tempfile::tempdir().expect("cannot create temp dir");
    let yard = yard::load_or_create(temp_folder.path())?;
    for app in apps {
        if let Some(start_app_name) = &start_at_app {
            if app.name() != start_app_name {
                continue;
            }
            start_at_app = None;
        }
        log(Event::IntegrationTestNewApp { app: &app.name() });
        let latest_version = app.latest_installable_version(log)?;
        log(Event::IntegrationTestDeterminedVersion {
            app: &app.name(),
            version: &latest_version,
        });
        for install_method in app.install_methods() {
            log(Event::IntegrationTestNewInstallMethod {
                version: &latest_version,
                method: &install_method,
            });
            let installed = install::install(&install_method, &latest_version, platform, &yard, log)?;
            if !installed {
                continue;
            }
            let executable_location = install_method.executable_location(&latest_version, platform);
            let executable_path = yard.app_folder(&install_method.yard_app(), &latest_version).join(executable_location);
            if !executable_path.exists() {
                println!(
                    "executable {} not found, press ENTER after inspecting the yard",
                    executable_path.to_string_lossy()
                );
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                return Ok(ExitCode::FAILURE);
            }
            let executable = Executable(executable_path.clone());
            match app.analyze_executable(&executable, log)? {
                AnalyzeResult::NotIdentified { output } => {
                    println!(
                        "executable {} not identified based on this output:\n\"{output}\"\nOUTPUT END",
                        executable_path.to_string_lossy()
                    );
                    return Ok(ExitCode::FAILURE);
                }
                AnalyzeResult::IdentifiedButUnknownVersion => println!("{}", "executable identified".green()),
                AnalyzeResult::IdentifiedWithVersion(executable_version) if executable_version == latest_version => {
                    println!("{}", "executable has the correct version".green());
                }
                AnalyzeResult::IdentifiedWithVersion(executable_version) => {
                    println!("executable has version {executable_version} but we installed version {latest_version}");
                    return Ok(ExitCode::FAILURE);
                }
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}
