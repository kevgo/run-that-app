use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{
    ArtifactType, CompileFromGoSource, DownloadPrecompiledBinary, InstallationMethod,
};
use crate::yard::Yard;

pub struct Shfmt {}

impl App for Shfmt {
    fn name(&self) -> &'static str {
        "shfmt"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "shfmt.exe",
            Os::Linux | Os::MacOS => "shfmt",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/mvdan/sh"
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
                url: format!("https://github.com/mvdan/sh/releases/download/v{version}/shfmt_v{version}_{os}_{cpu}", os = os_text(platform.os), cpu = cpu_text(platform.cpu)),
                artifact_type: ArtifactType::Executable,
                file_on_disk: yard.app_file_path(self.name(), version, self.executable(platform)),
            }),
            Box::new(CompileFromGoSource {
                import_path: format!("mvdan.cc/sh/v3/cmd/shfmt@{version}"),
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
