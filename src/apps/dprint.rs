use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};
use big_s::S;

pub struct Dprint {}

impl App for Dprint {
    fn name(&self) -> &'static str {
        "dprint"
    }

    fn executable(&self, platform: &Platform) -> &'static str {
        match platform.os {
            Os::Windows => "dprint.exe",
            Os::Linux | Os::MacOS => "dprint",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://dprint.dev"
    }

    fn online_location(&self, version: String, platform: &Platform) -> Box<dyn OnlineLocation> {
        Box::new(GithubReleaseAsset {
            organization: "dprint",
            repo: "dprint",
            version,
            filename: asset_filename(platform),
        })
    }

    fn file_to_extract_from_archive(&self, _version: &str, platform: &Platform) -> String {
        S(self.executable(platform))
    }
}

fn asset_filename(platform: &Platform) -> String {
    format!(
        "dprint-{cpu}-{os}.zip",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
    )
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
