use colored::Colorize;
use std::io::{self, Write};

use super::Event;

#[derive(Copy, Clone)]
pub struct Output {
    pub verbose: bool,
}

macro_rules! fprint {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        let _ = io::stderr().flush();
    }};
}

impl Output {
    pub fn log(&self, event: Event) {
        if self.verbose {
            display_verbose(event)
        } else {
            display_nonverbose(event)
        }
    }
}

fn display_verbose(event: Event) {
    match event {
        Event::CpuIdentified { architecture } => println!("CPU id: {architecture}"),
        Event::OsIdentified { name } => println!("OS id: {name}"),
        Event::DownloadBegin { app, url } => println!("start download {}", url.cyan().bold()),
        Event::DownloadSuccess => println!("finished download"),
        Event::DownloadFail => println!("download failed"),
        Event::ExtractBegin { archive_type: archive } => println!("extract {}", archive.cyan().bold()),
        Event::ExtractSuccess => println!("extract success"),
        Event::ExtractFail => println!("extract failed"),
        Event::CompileGoStart { go_path, args } => todo!(),
        Event::CompileGoSuccess => todo!(),
        Event::CompileRustStart { cargo_path, args } => todo!(),
        Event::CompileRustSuccess => todo!(),
    }
}

fn display_nonverbose(event: Event) {
    match event {
        Event::CpuIdentified { architecture } => {}
        Event::OsIdentified { name } => {}
        Event::DownloadBegin { app, url } => fprint!("downloading {} ... ", app.as_str().cyan().bold()),
        Event::DownloadSuccess => {}
        Event::DownloadFail => println!("{}", "failed".red().bold()),
        Event::ExtractBegin { archive_type: archive } => fprint!("extracting ..."),
        Event::ExtractSuccess => println!("{}", "ok".green().bold()),
        Event::ExtractFail => println!("{}", "failed".red().bold()),
        Event::CompileGoStart { go_path, args } => todo!(),
        Event::CompileGoSuccess => todo!(),
        Event::CompileRustStart { cargo_path, args } => todo!(),
        Event::CompileRustSuccess => todo!(),
    }
}
