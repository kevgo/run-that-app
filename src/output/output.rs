use colored::Colorize;
use std::io::{self, Write};

use super::Event;

#[derive(Copy, Clone)]
pub struct Output {
    pub verbose: bool,
}

impl Output {
    pub fn log(self, event: Event) {
        if self.verbose {
            display_verbose(event);
        } else {
            display_normal(event);
        }
    }
}

macro_rules! fprint {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        let _ = io::stderr().flush();
    }};
}

fn display_verbose(event: Event) {
    #[allow(clippy::match_same_arms)]
    match event {
        Event::ArchiveExtractBegin { archive_type } => fprint!("extracting {} ... ", archive_type.cyan()),
        Event::ArchiveExtractSuccess => eprintln!("{}", "ok".green()),
        Event::ArchiveExtractFailed { err } => eprintln!("{}", err.red()),

        Event::CompileGoBegin { go_path, args } => eprintln!("{go_path} {}", args.join(" ")),
        Event::CompileGoSuccess => eprintln!("Go compilation successful"),

        Event::CompileRustStart { cargo_path, args } => eprintln!("{} {}", cargo_path.to_string_lossy(), args.join(" ")),
        Event::CompileRustSuccess => eprintln!("Rust compilation successful"),

        Event::DownloadBegin { app: _, url } => fprint!("downloading {} ... ", url.cyan()),
        Event::DownloadSuccess => eprintln!("{}", "ok".green()),
        Event::DownloadFail { code } => eprintln!("{}", code.to_string().red()),
        Event::DownloadNotFound => eprintln!("{}", "not found".red()),

        Event::ExecutableInstallSaveBegin => fprint!("saving ... "),
        Event::ExecutableInstallSaveSuccess => eprintln!("{}", "ok".green()),
        Event::ExecutableInstallSaveFail { err } => eprintln!("{}", err.red()),

        Event::GitHubApiRequestBegin { url } => eprintln!("Talking to GitHub API: {url} ... "),
        Event::GitHubApiRequestSuccess => eprintln!("{}", "ok".green()),
        Event::GitHubApiRequestFail { err } => eprintln!("{}", err.red()),

        Event::GlobalInstallSearch { binary } => fprint!("Looking for {} in the PATH ... ", binary.cyan()),
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
                eprintln!("found version {} that does not match {}", version.as_str().red(), version_restriction.to_string().cyan());
            } else {
                eprintln!("found an installation with unknown version and it doesn't match {}", version_restriction.to_string().cyan());
            }
        }
        Event::GlobalInstallNotFound => eprintln!("{}", "not found".red()),
        Event::GlobalInstallNotIdentified => eprintln!("not found "),

        Event::IdentifiedCpu { architecture } => eprintln!("CPU id: {}", architecture.cyan()),
        Event::IdentifiedOs { name } => eprintln!("OS id: {}", name.cyan()),

        Event::NotOnline => eprintln!("{}", "not online".red()),

        Event::UpdateBegin { app } => eprintln!("updating {} ...", app.as_str().cyan()),
        Event::UpdateNewVersion { old_version, new_version } => eprintln!("{} -> {}", old_version.as_str().green(), new_version.as_str().green()),
        Event::UpdateAlreadyNewest { app: _ } => eprintln!("{}", "up to date".green()),
    }
}

fn display_normal(event: Event) {
    #[allow(clippy::match_same_arms)]
    match event {
        Event::ArchiveExtractBegin { archive_type: _ } => fprint!("extracting ... "),
        Event::ArchiveExtractSuccess => eprintln!("{}", "ok".green()),
        Event::ArchiveExtractFailed { err } => eprintln!("{}", err.red()),

        Event::CompileGoBegin { go_path: _, args } => eprintln!("go {}", args.join(" ")),
        Event::CompileGoSuccess => {}

        Event::CompileRustStart { cargo_path: _, args } => eprintln!("cargo {}", args.join(" ")),
        Event::CompileRustSuccess => {}

        Event::DownloadBegin { app, url: _ } => fprint!("downloading {} ... ", app.as_str().cyan()),
        Event::DownloadSuccess => {}
        Event::DownloadFail { code } => eprintln!("{}", code.to_string().red()),
        Event::DownloadNotFound => eprintln!("{}", "not found".red()),

        Event::ExecutableInstallSaveBegin => fprint!("saving ... "),
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

        Event::NotOnline => eprintln!("{}", "not online".red()),

        Event::UpdateBegin { app: _ } => {}
        Event::UpdateNewVersion { old_version, new_version } => eprintln!("{} -> {}", old_version.as_str().green(), new_version.as_str().green()),
        Event::UpdateAlreadyNewest { app: _ } => eprintln!("{}", "up to date".green()),
    }
}
