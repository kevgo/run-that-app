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
        Event::CpuIdentified { architecture } => println!("CPU id: {architecture}"),
        Event::OsIdentified { name } => println!("OS id: {name}"),
        Event::DownloadBegin { app, url } => println!("start download {}", url.cyan().bold()),
        Event::DownloadSuccess => println!("finished download"),
        Event::DownloadFail { code } => println!("download failed"),
        Event::ExtractBegin { archive_type: archive } => println!("extract {}", archive.cyan().bold()),
        Event::ExtractSuccess => println!("extract success"),
        Event::ExtractFail => println!("extract failed"),
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

fn display_normal(event: Event) {
    match event {
        Event::CpuIdentified { architecture } => {}
        Event::OsIdentified { name } => {}
        Event::DownloadBegin { app, url } => fprint!("downloading {} ... ", app.as_str().cyan().bold()),
        Event::DownloadSuccess => {}
        Event::DownloadFail { code } => println!("{}", code.to_string().red().bold()),
        Event::ExtractBegin { archive_type: archive } => fprint!("extracting ..."),
        Event::ExtractSuccess => println!("{}", "ok".green().bold()),
        Event::ExtractFail => println!("{}", "failed".red().bold()),
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
