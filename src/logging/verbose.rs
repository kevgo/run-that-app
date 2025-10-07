use super::Event;
use colored::Colorize;
use std::io::{self, Write};

/// a logger with verbose output, for debugging
pub(crate) fn log(event: Event) {
  #[allow(clippy::match_same_arms)]
  match event {
    Event::AnalyzeExecutableBegin { cmd, args } => eprintln!("{}", format!("{cmd} {}", args.join(" ")).bold()),

    Event::ArchiveExtractBegin { archive_type } => eprintf!("extracting {} ... ", archive_type.cyan()),
    Event::ArchiveExtractSuccess => eprintln!("{}", "ok".green()),
    Event::ArchiveExtractFailed { err } => eprintln!("{}", err.red()),

    Event::CompileGoBegin { go_path, args } => eprintln!("{go_path} {}", args.join(" ")),
    Event::CompileGoSuccess => eprintln!("{}", "Go compilation successful".green()),
    Event::CompileGoFailed => eprintln!("{}", "Go compilation failed".red()),

    Event::CompileRustStart { cargo_path, args } => eprintln!("{} {}", cargo_path.to_string_lossy(), args.join(" ")),
    Event::CompileRustSuccess => eprintln!("{}", "Rust compilation successful".green()),
    Event::CompileRustFailed => eprintln!("{}", "Rust compilation failed".red()),

    Event::DownloadBegin { app: _, url } => eprintf!("downloading {} ... ", url.as_ref().cyan()),
    Event::DownloadSuccess => eprintln!("{}", "ok".green()),
    Event::DownloadFail { code } => eprintln!("{}", code.to_string().red()),
    Event::DownloadNotFound { is_optional } => {
      if is_optional {
        eprintln!("{}", "not found, skipping".yellow());
      } else {
        eprintln!("{}", "not found".yellow());
      }
    }

    Event::ExecutableInstallSaveBegin => eprintf!("saving ... "),
    Event::ExecutableInstallSaveSuccess => eprintln!("{}", "ok".green()),
    Event::ExecutableInstallSaveFail { err } => eprintln!("{}", err.red()),

    Event::GitHubApiRequestBegin { url } => eprintf!("Talking to GitHub API ({url}) ... "),
    Event::GitHubApiRequestSuccess => eprintln!("{}", "ok".green()),
    Event::GitHubApiRequestFail { err } => eprintln!("{}", err.red()),

    Event::GlobalInstallSearch { binary } => eprintf!("Looking for {} in the PATH ... ", binary.to_string().cyan()),
    Event::GlobalInstallFound { path } => eprintln!("{}", path.to_string_lossy().green()),
    Event::GlobalInstallMatchingVersion { range: version_range, version } => {
      if let Some(version) = version {
        eprintln!("found version {} matching {}", version.as_str().green(), version_range.to_string().cyan());
      } else {
        eprintln!("found an installation with unknown version but it matches {}", version_range.to_string().cyan());
      }
    }
    Event::GlobalInstallMismatchingVersion {
      range: version_restriction,
      version: actual_version,
    } => {
      if let Some(version) = actual_version {
        eprintln!(
          "found version {} that does not match {}",
          version.as_str().red(),
          version_restriction.to_string().cyan()
        );
      } else {
        eprintln!(
          "found an installation with unknown version and it doesn't match {}",
          version_restriction.to_string().cyan()
        );
      }
    }
    Event::GlobalInstallNotFound => eprintln!("{}", "not found".red()),
    Event::GlobalInstallNotIdentified => eprintln!("not found "),

    Event::IdentifiedCpu { architecture } => eprintln!("CPU: {}", architecture.cyan()),
    Event::IdentifiedOs { name } => eprintln!("OS: {}", name.cyan()),

    Event::IntegrationTestNewApp { app } => eprintln!("TESTING {app}\n"),
    Event::IntegrationTestDeterminedVersion { version } => eprintln!("Latest version: {}", version.as_str().cyan()),
    Event::IntegrationTestNewInstallMethod { app, method, version } => eprintln!("\n{}", method.name(app, version).bold()),

    #[cfg(unix)]
    Event::MakeExecutable { file } => eprintln!("make file {} executable", file.to_string_lossy()),

    Event::NotOnline => eprintln!("{}", "not online".red()),

    Event::UpdateBegin { app } => eprintln!("updating {} ...", app.as_str().cyan()),
    Event::UpdateNewVersion { app, old_version, new_version } => eprintln!("{app}  {} -> {}", old_version.as_str().green(), new_version.as_str().green()),
    Event::UpdateAlreadyNewest { app } => eprintln!("{app}  {}", "up to date".green()),

    Event::YardCheckExistingAppBegin { path } => eprintf!("Checking for existing app {} ... ", path.to_string_lossy()),
    Event::YardCheckExistingAppFound => eprintln!("{}", "exists".green()),
    Event::YardCheckExistingAppNotFound => eprintln!("{}", "not found".red()),
  }
}
