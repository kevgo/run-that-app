use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{
    ArtifactType, CompileFromGoSource, DownloadPrecompiledBinary, InstallationMethod,
};
use crate::yard::Yard;

pub struct Gofumpt {}

impl App for Gofumpt {
    fn name(&self) -> &'static str {
        "gofumpt"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "gofumpt.exe",
            Os::Linux | Os::MacOS => "gofumpt",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/mvdan/gofumpt"
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
                url: format!("https://github.com/mvdan/gofumpt/releases/download/v{version}/gofumpt_v{version}_{os}_{cpu}", os = os_text(platform.os), cpu = cpu_text(platform.cpu)),
                artifact_type: ArtifactType::Executable,
                file_on_disk: yard.app_file_path(self.name(), version, self.executable(platform)),
            }),
            Box::new(CompileFromGoSource {
                import_path: format!("mvdan.cc/gofumpt@{version}"),
                target_folder: yard.app_folder(self.name(), version),
                executable_filename: self.executable(platform),
             }),
        ]
    }
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "darwin",
        Os::Windows => "windows",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "amd64",
    }
}
