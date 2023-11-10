use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};

pub struct Shfmt {}

impl App for Shfmt {
    fn name(&self) -> &'static str {
        "shfmt"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "shfmt.exe",
            Os::Linux | Os::MacOS => "shfmt",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/mvdan/sh"
    }

    fn artifact_location(&self, version: String, platform: Platform) -> Box<dyn OnlineLocation> {
        let filename = format!(
            "shfmt_{version}_{os}_{cpu}{ext}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
            ext = ext_text(platform.os),
        );
        Box::new(GithubReleaseAsset {
            organization: "mvdan",
            repo: "sh",
            version,
            filename,
        })
    }

    fn file_to_extract_from_archive(&self, _version: &str, _platform: Platform) -> String {
        String::new()
    }
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Windows => "windows",
        Os::Linux => "linux",
        Os::MacOS => "darwin",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Intel64 => "amd64",
        Cpu::Arm64 => "arm64",
    }
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Windows => ".exe",
        Os::Linux | Os::MacOS => "",
    }
}
