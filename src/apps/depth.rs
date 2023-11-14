use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{CompileFromGoSource, DownloadPrecompiledBinary};

pub struct Depth {}

impl App for Depth {
    fn name(&self) -> &'static str {
        "depth"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "depth.exe",
            Os::Linux | Os::MacOS => "depth",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/KyleBanks/depth"
    }

    fn installation_methods(
        &self,
        version: &str,
        platform: Platform,
        yard: &crate::yard::Yard,
    ) -> Vec<Box<dyn crate::install::InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                url: format!("https://github.com/KyleBanks/depth/releases/download/v{version}/depth_{version}_{os}_{cpu}", os = os_text(platform.os), cpu = cpu_text(platform.cpu)),
                file_in_archive: None,
                file_on_disk: yard.app_file_path(self.name(), version, self.executable(platform)),
            }),
            Box::new(CompileFromGoSource {
                import_path: format!("github.com/KyleBanks/depth/cmd/depth@v{version}"),
                executable_filename: self.executable(platform),
                target_folder: yard.app_folder(self.name(), version),
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
        Cpu::Arm64 => "arm",
        Cpu::Intel64 => "amd64",
    }
}