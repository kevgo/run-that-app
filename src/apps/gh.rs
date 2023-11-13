use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};

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
            organization: "cli",
            repo: "cli",
            version: format!("v{version}"),
            filename,
        })
    }

    fn file_to_extract_from_archive(
        &self,
        version: &str,
        Platform { os, cpu }: Platform,
    ) -> Option<String> {
        Some(format!("gh_{version}_{os}_{cpu}/bin/gh",))
    }
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "macOS",
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
        Os::Linux => "tgz",
        Os::Windows | Os::MacOS => "zip",
    }
}
