use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{
    ArtifactType, CompileFromGoSource, DownloadPrecompiledBinary, InstallationMethod,
};
use crate::yard::Yard;

pub struct Goreleaser {}

impl App for Goreleaser {
    fn name(&self) -> &'static str {
        "goreleaser"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "goreleaser.exe",
            Os::Linux | Os::MacOS => "goreleaser",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://goreleaser.com"
    }

    fn installation_methods(
        &self,
        version: &str,
        platform: Platform,
        yard: &Yard,
    ) -> Vec<Box<dyn InstallationMethod>> {
        let os = os_text(platform.os);
        let cpu = cpu_text(platform.cpu);
        let ext = ext_text(platform.os);
        vec![
            Box::new(DownloadPrecompiledBinary {
                name: self.name(),
                url: format!("https://github.com/goreleaser/goreleaser/releases/download/v{version}/goreleaser_{os}_{cpu}.{ext}" ),
                artifact_type: ArtifactType::Archive { file_to_extract: self.executable_filename(platform).to_string() },
                file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            }),
            Box::new(CompileFromGoSource {
                import_path: format!("github.com/goreleaser/goreleaser@{version}"),
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

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux | Os::MacOS => "tar.gz",
        Os::Windows => "zip",
    }
}
