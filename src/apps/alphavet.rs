use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};

pub struct Alphavet {}

impl App for Alphavet {
    fn name(&self) -> &'static str {
        "alphavet"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "alphavet.exe",
            Os::Linux | Os::MacOS => "alphavet",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/skx/alphavet"
    }

    fn artifact_location(&self, version: &str, platform: Platform) -> Box<dyn OnlineLocation> {
        let filename = format!(
            "alphavet-{os}-{cpu}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
        );
        Box::new(GithubReleaseAsset {
            organization: "skx",
            repo: "alphavet",
            version: version.to_string(),
            filename,
        })
    }

    fn file_to_extract_from_archive(&self, _version: &str, _platform: Platform) -> Option<String> {
        None
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
