use crate::apps::AnalyzeResult;
use crate::config::AppName;
use crate::install;
use crate::subshell::Executable;
use crate::{apps, logger, platform, yard, Result};
use colored::Colorize;
use std::process::ExitCode;

pub fn test(want_app: &Option<AppName>, verbose: bool) -> Result<ExitCode> {
    let apps = apps::all();
    let log = logger::new(verbose);
    let platform = platform::detect(log)?;
    let temp_folder = tempfile::tempdir().expect("cannot create temp dir");
    let yard = yard::load_or_create(temp_folder.path())?;
    for app in apps {
        if let Some(want_app) = want_app {
            if app.name() != want_app {
                continue;
            }
        }
        println!("\n\nTESTING {}", app.name().as_str().cyan());
        let latest_version = app.latest_installable_version(log)?;
        for install_method in app.install_methods() {
            println!("\n{}", install_method.to_string().bold());
            let installed = install::install(&install_method, &latest_version, platform, &yard, log)?;
            if !installed {
                continue;
            }
            let executable_location = install_method.executable_location(&latest_version, platform);
            let executable_path = yard.app_folder(&install_method.yard_app(), &latest_version).join(executable_location);
            if !executable_path.exists() {
                println!("executable {} not found", executable_path.to_string_lossy());
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
                AnalyzeResult::IdentifiedWithVersion(executable_version) if executable_version == latest_version => println!("{}", "executable has the correct version".green()),
                AnalyzeResult::IdentifiedWithVersion(executable_version) => {
                    println!("executable has version {executable_version} but we installed version {latest_version}");
                    return Ok(ExitCode::FAILURE);
                }
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}
