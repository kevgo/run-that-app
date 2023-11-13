use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};

pub struct GolangCiLint {}

impl App for GolangCiLint {
    fn name(&self) -> &'static str {
        "golangci-lint"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "golangci-lint.exe",
            Os::Linux | Os::MacOS => "golangci-lint",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/golangci/golangci-lint"
    }

    fn artifact_location(&self, version: &str, platform: Platform) -> Box<dyn OnlineLocation> {
        let filename = format!(
            "golangci-lint-v{version}-{os}-{cpu}.{ext}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
            ext = ext_text(platform.os),
        );
        Box::new(GithubReleaseAsset {
            organization: "golangci",
            repo: "golangci-lint",
            version: format!("v{version}"),
            filename,
        })
    }

    fn file_to_extract_from_archive(&self, _version: &str, _platform: Platform) -> Option<String> {
        None
    }
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "darwin",
        Os::Windows => "windows",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "amd64",
    }
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux | Os::MacOS => "tar.gz",
        Os::Windows => "zip",
    }
}
