use colored::Colorize;
use std::io::{self, Write};

use super::Event;

pub struct Output {
    verbose: bool,
}

macro_rules! fprint {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        let _ = io::stderr().flush();
    }};
}

impl Output {
    pub fn display(&self, event: Event) {
        if self.verbose {
            display_verbose(event)
        } else {
            display_nonverbose(event)
        }
    }
}

fn display_verbose(event: Event) {
    match event {
        Event::DownloadBegin { app, url } => println!("start download {}", url.cyan().bold()),
        Event::DownloadSuccess => println!("finished download"),
        Event::DownloadFail => println!("download failed"),
        Event::ExtractBegin { archive } => println!("extract {}", archive.cyan().bold()),
        Event::ExtractSuccess => println!("extract success"),
        Event::ExtractFail => println!("extract failed"),
    }
}

fn display_nonverbose(event: Event) {
    match event {
        Event::DownloadBegin { app, url } => fprint!("downloading {} ... ", app.as_str().cyan().bold()),
        Event::DownloadSuccess => {}
        Event::DownloadFail => println!("{}", "failed".red().bold()),
        Event::ExtractBegin { archive } => fprint!("extracting ..."),
        Event::ExtractSuccess => println!("{}", "ok".green().bold()),
        Event::ExtractFail => println!("{}", "failed".red().bold()),
    }
}
