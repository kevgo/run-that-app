use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{CompileFromGoSource, DownloadPrecompiledBinary};
use crate::yard::Yard;

pub struct Alphavet {}

impl App for Alphavet {
    fn name(&self) -> &'static str {
        "alphavet"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "alphavet.exe",
            Os::Linux | Os::MacOS => "alphavet",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/skx/alphavet"
    }

    fn installation_methods(
        &self,
        version: &str,
        platform: Platform,
        yard: &Yard,
    ) -> Vec<Box<dyn crate::install::InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                url: format!("https://github.com/skx/alphavet/releases/download/v{version}/alphavet-{os}-{cpu}", os = os_text(platform.os), cpu = cpu_text(platform.cpu)),
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
