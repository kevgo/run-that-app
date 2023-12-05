use super::App;
use crate::hosting::github;
use crate::install::{ArtifactType, CompileFromGoSource, DownloadPrecompiledBinary, InstallationMethod};
use crate::output::Output;
use crate::platform::{Cpu, Os, Platform};
use crate::yard::Yard;
use crate::Result;

pub struct ActionLint {}

impl App for ActionLint {
    fn name(&self) -> &'static str {
        "actionlint"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "actionlint.exe",
            Os::Linux | Os::MacOS => "actionlint",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://rhysd.github.io/actionlint"
    }

    fn installation_methods(&self, version: &str, platform: Platform, yard: &Yard) -> Vec<Box<dyn InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                name: self.name(),
                url: download_url(version, platform),
                artifact_type: ArtifactType::Archive {
                    file_to_extract: self.executable_filename(platform).to_string(),
                },
                file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            }),
            Box::new(CompileFromGoSource {
                import_path: format!("github.com/rhysd/actionlint/cmd/actionlint@{version}"),
                target_folder: yard.app_folder(self.name(), version),
                executable_filename: self.executable_filename(platform),
            }),
        ]
    }

    fn versions(&self, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
        github::versions("rhysd", "actionlint", amount, output)
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/rhysd/actionlint/releases/download/v{version}/actionlint_{version}_{os}_{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
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
        Os::Linux | Os::MacOS => "tar.gz",
        Os::Windows => "zip",
    }
}

#[cfg(test)]
mod tests {
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform {
            os: Os::Linux,
            cpu: Cpu::Arm64,
        };
        let have = super::download_url("1.6.26", platform);
        let want = "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_linux_arm64.tar.gz";
        assert_eq!(have, want);
    }
}
