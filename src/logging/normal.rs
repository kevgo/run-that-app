use super::Event;
use colored::Colorize;
use std::io::{self, Write};

/// a logger with concise output, for normal production use
pub(crate) fn log(event: Event) {
  #[allow(clippy::match_same_arms)]
  match event {
    Event::AnalyzeExecutableBegin { cmd: _, args: _ } => {}

    Event::ArchiveExtractBegin { archive_type: _ } => eprintf!("extracting ... "),
    Event::ArchiveExtractSuccess => eprintln!("{}", "ok".green()),
    Event::ArchiveExtractFailed { err } => eprintln!("{}", err.red()),

    Event::CompileGoBegin { go_path: _, args } => eprintln!("go {}", args.join(" ")),
    Event::CompileGoSuccess => {}
    Event::CompileGoFailed => eprintln!("{}", "Go compilation failed".red()),

    Event::CompileRustStart { cargo_path: _, args } => eprintln!("cargo {}", args.join(" ")),
    Event::CompileRustSuccess => {}
    Event::CompileRustFailed => eprintln!("{}", "Rust compilation failed".red()),

    Event::DownloadBegin { app, url: _ } => eprintf!("downloading {} ... ", app.as_str().cyan()),
    Event::DownloadSuccess => {}
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

    Event::GitHubApiRequestBegin { url: _ } => {}
    Event::GitHubApiRequestFail { err } => eprintln!("GitHub API request failed: {}", err.red()),
    Event::GitHubApiRequestSuccess => {}

    Event::GlobalInstallSearch { binary: _ } => {}
    Event::GlobalInstallFound { path: _ } => {}
    Event::GlobalInstallNotFound => {}
    Event::GlobalInstallMatchingVersion { range: _, version: _ } => {}
    Event::GlobalInstallMismatchingVersion { range: _, version: _ } => {}
    Event::GlobalInstallNotIdentified => {}

    Event::IdentifiedCpu { architecture: _ } => {}
    Event::IdentifiedOs { name: _ } => {}

    Event::IntegrationTestNewApp { app: _ } => {}
    Event::IntegrationTestDeterminedVersion { version: _ } => {}
    Event::IntegrationTestNewInstallMethod { app, method, version } => eprintln!("\n{}", method.name(app, version).bold()),

    #[cfg(unix)]
    Event::MakeExecutable { file: _ } => {}
    Event::NotOnline => eprintln!("{}", "not online".red()),

    Event::UpdateBegin { app: _ } => {}
    Event::UpdateNewVersion { app, old_version, new_version } => eprintln!("{app}  {} -> {}", old_version.as_str().green(), new_version.as_str().green()),
    Event::UpdateAlreadyNewest { app } => eprintln!("{app}  {}", "up to date".green()),

    Event::YardCheckExistingAppBegin { path: _ } => {}
    Event::YardCheckExistingAppFound => {}
    Event::YardCheckExistingAppNotFound => {}
  }
}
