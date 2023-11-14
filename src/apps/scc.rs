use big_s::S;

use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{CompileFromGoSource, DownloadPrecompiledBinary, InstallationMethod};
use crate::yard::Yard;

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

    fn installation_methods(
        &self,
        version: &str,
        platform: Platform,
        yard: &Yard,
    ) -> Vec<Box<dyn InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                url: format!("https://github.com/boyter/scc/releases/download/v{version}/scc_{version}_{os}_{cpu}.{ext}", os = os_text(platform.os), cpu = cpu_text(platform.cpu), ext = ext_text(platform.os)),
                file_in_archive: Some(S(self.executable(platform))),
                file_on_disk: yard.app_file_path(self.name(), version, self.executable(platform)),
            }),
            Box::new(CompileFromGoSource {
                import_path: format!("github.com/boyter/scc/v3@{version}"),
                target_folder: yard.app_folder(self.name(), version),
                executable_filename: self.executable(platform),
             }),
        ]
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
