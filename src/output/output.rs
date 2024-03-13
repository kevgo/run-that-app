use colored::Colorize;
use std::io::{self, Write};

use super::Event;

#[derive(Copy, Clone)]
pub struct Output {
    pub verbose: bool,
}

impl Output {
    pub fn log(&self, event: Event) {
        if self.verbose {
            display_verbose(event)
        } else {
            display_normal(event)
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
    match event {
        Event::ArchiveExtractBegin { archive_type } => fprint!("extracting {} ... ", archive_type.cyan().bold()),
        Event::ArchiveExtractSuccess => eprintln!("{}", "ok".green().bold()),
        Event::ArchiveExtractFailed { err } => eprintln!("{}", err.red().bold()),

        Event::CpuIdentified { architecture } => eprintln!("CPU id: {}", architecture.cyan().bold()),
        Event::OsIdentified { name } => eprintln!("OS id: {}", name.cyan().bold()),

        Event::DownloadBegin { app, url } => eprintln!("download {} from {}", app.as_str().cyan().bold(), url.cyan()),
        Event::DownloadSuccess => eprintln!("{}", "ok".green().bold()),
        Event::DownloadFail { code } => eprintln!("{}", code.to_string().red().bold()),
        Event::DownloadNotFound => eprintln!("{}", "not found".red().bold()),

        Event::CompileGoStart { go_path, args } => eprintln!("{go_path} {}", args.join(" ")),
        Event::CompileGoSuccess => eprintln!("Go compilation successful"),

        Event::CompileRustStart { cargo_path, args } => eprintln!("{} {}", cargo_path.to_string_lossy(), args.join(" ")),
        Event::CompileRustSuccess => eprintln!("Rust compilation successful"),

        Event::ExecutableInstallSave => fprint!("saving ... "),
        Event::ExecutableInstallSaveSuccess => eprintln!("{}", "ok".green().bold()),
        Event::ExecutableInstallSaveFail { err } => eprintln!("{}", err.red().bold()),

        Event::GitHubApiRequestBegin { url } => eprintln!("Talking to GitHub API: {url} ... "),
        Event::GitHubApiRequestSuccess => eprintln!("{}", "ok".green().bold()),
        Event::GitHubApiRequestFail { err } => eprintln!("{}", err.red().bold()),

        Event::GlobalInstallSearch { binary } => fprint!("Looking for {} in the PATH ... ", binary.cyan().bold()),
        Event::GlobalInstallFound { path } => eprintln!("{}", path.to_string_lossy().green().bold()),
        Event::GlobalInstallMatchingVersion {
            version_restriction,
            actual_version,
        } => {
            if let Some(version) = actual_version {
                eprintln!(
                    "found version {} matching {}",
                    version.as_str().green().bold(),
                    version_restriction.to_string().cyan().bold()
                );
            } else {
                eprintln!(
                    "found an installation with unknown version but it matches {}",
                    version_restriction.to_string().cyan().bold()
                );
            }
        }
        Event::GlobalInstallMismatchingVersion {
            version_restriction,
            actual_version,
        } => {
            if let Some(version) = actual_version {
                eprintln!(
                    "found version {} that does not match {}",
                    version.as_str().red().bold(),
                    version_restriction.to_string().cyan().bold()
                );
            } else {
                eprintln!(
                    "found an installation with unknown version and it doesn't match {}",
                    version_restriction.to_string().cyan().bold()
                );
            }
        }
        Event::GlobalInstallNotFound => eprintln!("{}", "not found".red().bold()),
        Event::GlobalInstallNotIdentified => eprintln!("not found "),

        Event::NotOnline => eprintln!("{}", "not online".red().bold()),

        Event::UpdateBegin { app } => eprintln!("updating {} ...", app.as_str().cyan().bold()),
        Event::UpdateNewVersion { old_version, new_version } => eprintln!("{} -> {}", old_version.as_str().green().bold(), new_version.as_str().green().bold()),
        Event::UpdateAlreadyNewest { app } => eprintln!("{}", "up to date".green().bold()),
    }
}

fn display_normal(event: Event) {
    match event {
        Event::ArchiveExtractBegin { archive_type: _ } => fprint!("extracting ... "),
        Event::ArchiveExtractSuccess => eprintln!("{}", "ok".green().bold()),
        Event::ArchiveExtractFailed { err } => eprintln!("{}", err.red().bold()),

        Event::CpuIdentified { architecture } => {}
        Event::OsIdentified { name } => {}

        Event::DownloadBegin { app, url } => fprint!("downloading {} ... ", app.as_str().cyan().bold()),
        Event::DownloadSuccess => {}
        Event::DownloadFail { code } => println!("{}", code.to_string().red().bold()),
        Event::DownloadNotFound => todo!(),

        Event::CompileGoStart { go_path, args } => todo!(),
        Event::CompileGoSuccess => todo!(),

        Event::CompileRustStart { cargo_path, args } => todo!(),
        Event::CompileRustSuccess => todo!(),

        Event::GitHubApiRequestBegin { url } => todo!(),
        Event::GitHubApiRequestFail { err } => todo!(),
        Event::GitHubApiRequestSuccess => todo!(),

        Event::GlobalInstallSearch { binary } => todo!(),
        Event::GlobalInstallFound { path } => todo!(),
        Event::GlobalInstallNotFound => todo!(),
        Event::GlobalInstallMatchingVersion {
            version_restriction,
            actual_version,
        } => todo!(),
        Event::GlobalInstallMismatchingVersion {
            version_restriction,
            actual_version,
        } => todo!(),
        Event::GlobalInstallNotIdentified => todo!(),

        Event::ExecutableInstallSave => todo!(),
        Event::ExecutableInstallSaveSuccess => todo!(),
        Event::ExecutableInstallSaveFail { err } => todo!(),

        Event::NotOnline => todo!(),

        Event::UpdateBegin { app } => todo!(),
        Event::UpdateNewVersion {
            old_version,
            new_version: new_versin,
        } => todo!(),
        Event::UpdateAlreadyNewest { app } => todo!(),
    }
}
