use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{DownloadPrecompiledBinary, InstallationMethod};
use crate::yard::Yard;

pub struct Gh {}

impl App for Gh {
    fn name(&self) -> &'static str {
        "gh"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "dprint.exe",
            Os::Linux | Os::MacOS => "dprint",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://cli.github.com"
    }

    fn installation_methods(
        &self,
        version: &str,
        platform: Platform,
        yard: &Yard,
    ) -> Vec<Box<dyn InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                url: format!("https://github.com/cli/cli/releases/download/v{version}/gh_{version}_{os}_{cpu}.{ext}", os = os_text(platform.os), cpu = cpu_text(platform.cpu), ext =ext_text(platform.os)),
                file_in_archive: Some(format!("gh_{version}_{os}_{cpu}/bin/gh", os=os_text(platform.os), cpu = cpu_text(platform.cpu))),
                file_on_disk: yard.app_file_path(self.name(), version, self.executable(platform)),
            }),
        ]
    }
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "macOS",
        Os::Windows => "windows",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "amd64",
    }
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "tgz",
        Os::Windows | Os::MacOS => "zip",
    }
}
