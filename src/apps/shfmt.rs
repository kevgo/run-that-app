use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::{CompileFromGoSource, DownloadPrecompiledBinary, InstallationMethod};

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
        yard: &crate::yard::Yard,
    ) -> Vec<Box<dyn InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                url: format!("https://github.com/koalaman/shellcheck/releases/download/v{version}/shellcheck-v{version}.{os}.{cpu}.{ext}", os = os_text(platform.os), cpu = cpu_text(platform.cpu), ext = ext_text(platform.os)),
                file_in_archive: None,
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

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux | Os::MacOS => "",
        Os::Windows => ".exe",
    }
}
