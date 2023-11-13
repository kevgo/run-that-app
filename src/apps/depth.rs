use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};

pub struct Depth {}

impl App for Depth {
    fn name(&self) -> &'static str {
        "depth"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "depth.exe",
            Os::Linux | Os::MacOS => "depth",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/KyleBanks/depth"
    }

    fn artifact_location(&self, version: &str, platform: Platform) -> Box<dyn OnlineLocation> {
        let filename = format!(
            "depth_{version}_{os}_{cpu}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
        );
        Box::new(GithubReleaseAsset {
            organization: "KyleBanks",
            repo: "depth",
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
        Cpu::Arm64 => "arm",
        Cpu::Intel64 => "amd64",
    }
}
