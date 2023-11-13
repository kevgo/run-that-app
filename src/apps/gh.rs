use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};
use big_s::S;

pub struct Gh {}

impl App for Gh {
    fn name(&self) -> &'static str {
        "gh"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "dprint.exe",
            Os::Linux | Os::MacOS => "dprint",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://cli.github.com"
    }

    fn artifact_location(&self, version: &str, platform: Platform) -> Box<dyn OnlineLocation> {
        let filename = format!(
            "gh_{version}_{os}_{cpu}.{ext}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
            ext = ext_text(platform.os)
        );
        Box::new(GithubReleaseAsset {
            organization: "dprint",
            repo: "dprint",
            version: version.to_string(),
            filename,
        })
    }

    fn file_to_extract_from_archive(&self, _version: &str, platform: Platform) -> Option<String> {
        Some(S(self.executable(platform)))
    }
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Windows => "windows",
        Os::Linux => "linux",
        Os::MacOS => "macOS",
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
        Os::Windows | Os::MacOS => "zip",
        Os::Linux => "tgz",
    }
}
