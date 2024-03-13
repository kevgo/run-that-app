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
        Event::ArchiveExtractBegin { archive_type } => todo!(),
        Event::ArchiveExtractSuccess => todo!(),
        Event::ArchiveExtractFailed { err } => todo!(),

        Event::CpuIdentified { architecture } => eprintln!("CPU id: {architecture}"),
        Event::OsIdentified { name } => eprintln!("OS id: {name}"),

        Event::DownloadBegin { app, url } => eprintln!("download {} from {}", app, url.cyan().bold()),
        Event::DownloadSuccess => eprintln!("finished download"),
        Event::DownloadFail { code } => eprintln!("download failed: {}", code.to_string().red().bold()),
        Event::DownloadNotFound => eprintln!("not found"),

        Event::CompileGoStart { go_path, args } => eprintln!("{go_path} {}", args.join(" ")),
        Event::CompileGoSuccess => eprintln!("Go compilation successful"),
        Event::CompileRustStart { cargo_path, args } => eprintln!("{} {}", cargo_path.to_string_lossy(), args.join(" ")),
        Event::CompileRustSuccess => eprintln!("Rust compilation successful"),

        Event::ExecutableInstallSave => fprint!("saving ... "),
        Event::ExecutableInstallSaveSuccess => eprintln!("{}", "ok".green().bold()),
        Event::ExecutableInstallSaveFail => eprintln!("{}", "failed".red().bold()),

        Event::GitHubApiRequestBegin { url } => eprintln!("Talking to GitHub API: {url}"),
        Event::GitHubApiRequestSuccess => eprintln!("GitHub API request success"),
        Event::GitHubApiRequestFail { err } => eprintln!("GitHub API request failed: {err}"),

        Event::GlobalInstallSearch { binary } => fprint!("Looking for {} in the PATH ... ", binary.cyan().bold()),
        Event::GlobalInstallFound { path } => eprintln!("{}", path.to_string_lossy().green().bold()),
        Event::GlobalInstallMatchingVersion {
            version_restriction,
            actual_version,
        } => todo!(),
        Event::GlobalInstallMismatchingVersion {
            version_restriction,
            actual_version,
        } => todo!(),
        Event::GlobalInstallNotFound => eprintln!("{}", "not found".red().bold()),
        Event::GlobalInstallNotIdentified { executable } => todo!(),

        Event::NotOnline => eprintln!("{}", "not online".red().bold()),

        Event::UpdateBegin { app } => todo!(),
        Event::UpdateNewVersion { app, old_version, new_versin } => todo!(),
        Event::UpdateAlreadyNewest { app } => todo!(),
    }
}

fn display_normal(event: Event) {
    match event {
        Event::CpuIdentified { architecture } => {}
        Event::OsIdentified { name } => {}
        Event::DownloadBegin { app, url } => fprint!("downloading {} ... ", app.as_str().cyan().bold()),
        Event::DownloadSuccess => {}
        Event::DownloadFail { code } => println!("{}", code.to_string().red().bold()),
        Event::NotOnline => todo!(),
        Event::DownloadNotFound => todo!(),
        Event::CompileGoStart { go_path, args } => todo!(),
        Event::CompileGoSuccess => todo!(),
        Event::CompileRustStart { cargo_path, args } => todo!(),
        Event::CompileRustSuccess => todo!(),
        Event::GitHubApiRequestBegin { url } => todo!(),
        Event::GitHubApiRequestFail { err } => todo!(),
        Event::GlobalInstallSearch { binary } => todo!(),
        Event::GlobalInstallFound { path } => todo!(),
        Event::GlobalInstallNotFound => todo!(),
        Event::ExecutableInstallSave => todo!(),
        Event::ExecutableInstallSaveSuccess => todo!(),
        Event::ExecutableInstallSaveFail => todo!(),
        Event::GitHubApiRequestSuccess => todo!(),
        Event::UpdateBegin { app } => todo!(),
        Event::UpdateNewVersion { app, old_version, new_versin } => todo!(),
        Event::UpdateAlreadyNewest { app } => todo!(),
        Event::ArchiveExtractBegin { archive_type } => todo!(),
        Event::ArchiveExtractSuccess => todo!(),
        Event::ArchiveExtractFailed { err } => todo!(),
        Event::GlobalInstallMatchingVersion {
            version_restriction,
            actual_version,
        } => todo!(),
        Event::GlobalInstallMismatchingVersion {
            version_restriction,
            actual_version,
        } => todo!(),
        Event::GlobalInstallNotIdentified { executable } => todo!(),
    }
}
