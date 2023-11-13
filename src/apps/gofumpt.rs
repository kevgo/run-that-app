use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};
use big_s::S;

pub struct Gofumpt {}

impl App for Gofumpt {
    fn name(&self) -> &'static str {
        "gofumpt"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "dprint.exe",
            Os::Linux | Os::MacOS => "dprint",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://dprint.dev"
    }

    fn artifact_location(&self, version: &str, platform: Platform) -> Box<dyn OnlineLocation> {
        let filename = format!(
            "dprint-{cpu}-{os}.zip",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
        );
        Box::new(GithubReleaseAsset {
            organization: "dprint",
            repo: "dprint",
            version: version.to_string(),
            filename,
        })
    }

    fn file_to_extract_from_archive(&self, _version: &str, platform: Platform) -> String {
        S(self.executable(platform))
    }
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Windows => "pc-windows-msvc",
        Os::Linux => "unknown-linux-gnu",
        Os::MacOS => "apple-darwin",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Intel64 => "x86_64",
        Cpu::Arm64 => "aarch64",
    }
}
