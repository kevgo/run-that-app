use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{CompileFromRustSource, DownloadPrecompiledBinary, InstallationMethod};
use crate::yard::Yard;
use big_s::S;

pub struct Dprint {}

impl App for Dprint {
    fn name(&self) -> &'static str {
        "dprint"
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

    fn installation_methods(
        &self,
        version: &str,
        platform: Platform,
        yard: &Yard,
    ) -> Vec<Box<dyn InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                url: format!("https://github.com/dprint/dprint/releases/download/{version}/dprint-{cpu}-{os}.zip", os = os_text(platform.os), cpu = cpu_text(platform.cpu)),
                file_in_archive: Some(S(self.executable(platform))),
                file_on_disk: yard.app_file_path(self.name(), version, self.executable(platform)),
            }),
            Box::new(CompileFromRustSource {
                crate_name: "dprint",
                target_folder: yard.app_folder(self.name(), version),
                executable_filename: self.executable(platform),
            }),
        ]
    }
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "unknown-linux-gnu",
        Os::MacOS => "apple-darwin",
        Os::Windows => "pc-windows-msvc",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "aarch64",
        Cpu::Intel64 => "x86_64",
    }
}
