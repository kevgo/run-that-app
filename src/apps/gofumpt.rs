use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{CompileFromGoSource, DownloadPrecompiledBinary};

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
        yard: &crate::yard::Yard,
    ) -> Vec<Box<dyn crate::install::InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                url: format!("https://github.com/mvdan/gofumpt/releases/download/v{version}/gofumpt_v{version}_{os}_{cpu}", os = os_text(platform.os), cpu = cpu_text(platform.cpu)),
                file_in_archive: None,
                file_on_disk: yard.app_file_path(self.name(), version, self.executable(platform)),
            }),
            Box::new(CompileFromGoSource {}),
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
