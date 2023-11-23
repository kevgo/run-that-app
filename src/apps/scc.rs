use big_s::S;

use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{
    ArtifactType, CompileFromGoSource, DownloadPrecompiledBinary, InstallationMethod,
};
use crate::yard::Yard;

pub struct Scc {}

impl App for Scc {
    fn name(&self) -> &'static str {
        "scc"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
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
                name: self.name(),
                url: format!("https://github.com/boyter/scc/releases/download/v{version}/scc_{version}_{os}_{cpu}.{ext}", os = os_text(platform.os), cpu = cpu_text(platform.cpu), ext = ext_text(platform.os)),
                artifact_type: ArtifactType::Archive { file_to_extract: S(self.executable_filename(platform))},
                file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            }),
            Box::new(CompileFromGoSource {
                import_path: format!("github.com/boyter/scc/v3@{version}"),
                target_folder: yard.app_folder(self.name(), version),
                executable_filename: self.executable_filename(platform),
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
