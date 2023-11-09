use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineAsset};
use big_s::S;

pub struct Dprint {}

impl App for Dprint {
    fn executable(&self) -> &'static str {
        "dprint"
    }

    fn homepage(&self) -> &'static str {
        "https://dprint.dev"
    }

    fn online_asset(&self, version: String, platform: &Platform) -> Box<dyn OnlineAsset> {
        Box::new(GithubReleaseAsset {
            organization: String::from("dprint"),
            repo: String::from("dprint"),
            version,
            filename: artifact_filename(platform),
        })
    }

    fn file_to_extract_from_archive(&self, version: &str) -> String {
        S("dprint")
    }
}

fn artifact_filename(platform: &Platform) -> String {
    format!(
        "dprint-{cpu}-{os}.zip",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
    )
}

fn path_in_archive(platform: Platform) -> String {
    String::from(match platform.os {
        Os::Windows => "dprint.exe",
        Os::Linux | Os::MacOS => "dprint",
    })
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
