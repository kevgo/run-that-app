use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::hosting::{GithubReleaseAsset, OnlineLocation};
use big_s::S;

pub struct ShellCheck {}

impl App for ShellCheck {
    fn name(&self) -> &'static str {
        "shellcheck"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "shellcheck.exe",
            Os::Linux | Os::MacOS => "shellcheck",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://www.shellcheck.net"
    }

    fn artifact_location(&self, version: String, platform: Platform) -> Box<dyn OnlineLocation> {
        let filename = format!(
            "shellcheck-{version}.{os}.{cpu}.{ext}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
            ext = ext_text(platform.os),
        );
        Box::new(GithubReleaseAsset {
            organization: "koalaman",
            repo: "shellcheck",
            version,
            filename,
        })
    }

    fn file_to_extract_from_archive(&self, _version: &str, platform: Platform) -> String {
        S(self.executable(platform))
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
        Cpu::Intel64 => "x86_64",
        Cpu::Arm64 => "aarch64",
    }
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Windows => "zip",
        Os::Linux | Os::MacOS => "tar.gz",
    }
}
