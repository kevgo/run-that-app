use big_s::S;

use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};

pub struct Scc {}

impl App for Scc {
    fn name(&self) -> &'static str {
        "scc"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "scc.exe",
            Os::Linux | Os::MacOS => "scc",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/boyter/scc"
    }

    fn artifact_location(&self, version: &str, platform: Platform) -> Box<dyn OnlineLocation> {
        let filename = format!(
            "scc_{version}_{os}_{cpu}.{ext}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
            ext = ext_text(platform.os)
        );
        Box::new(GithubReleaseAsset {
            organization: "boyter",
            repo: "scc",
            version: format!("v{version}"),
            filename,
        })
    }

    fn file_to_extract_from_archive(&self, _version: &str, platform: Platform) -> Option<String> {
        Some(S(self.executable(platform)))
    }
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "Linux",
        Os::MacOS => "Darwin",
        Os::Windows => "Windows",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "x86_64",
    }
}

fn ext_text(_os: Os) -> &'static str {
    "tar.gz"
}
