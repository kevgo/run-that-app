use super::App;
use crate::hosting::github;
use crate::install::{ArtifactType, DownloadPrecompiledBinary, InstallationMethod};
use crate::output::Output;
use crate::platform::{Cpu, Os, Platform};
use crate::yard::Yard;
use crate::Result;

pub struct Gh {}

impl App for Gh {
    fn name(&self) -> &'static str {
        "gh"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "gh.exe",
            Os::Linux | Os::MacOS => "gh",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://cli.github.com"
    }

    fn installation_methods(&self, version: &str, platform: Platform, yard: &Yard) -> Vec<Box<dyn InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                name: self.name(),
                url: download_url(version, platform),
                artifact_type: ArtifactType::Archive {
                    file_to_extract: executable_path(version, platform),
                },
                file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            }),
            // installation from source seems more involved, see https://github.com/cli/cli/blob/trunk/docs/source.md
        ]
    }

    fn versions(&self, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
        github::versions("cli", "cli", amount, output)
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/cli/cli/releases/download/v{version}/gh_{version}_{os}_{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

fn executable_path(version: &str, platform: Platform) -> String {
    match platform.os {
        Os::Windows => "bin/gh.exe".to_string(),
        Os::Linux | Os::MacOS => format!("gh_{version}_{os}_{cpu}/bin/gh", os = os_text(platform.os), cpu = cpu_text(platform.cpu)),
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
        Os::Linux => "tar.gz",
        Os::Windows | Os::MacOS => "zip",
    }
}

#[cfg(test)]
mod tests {
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform {
            os: Os::Linux,
            cpu: Cpu::Intel64,
        };
        let have = super::download_url("2.39.1", platform);
        let want = "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_linux_amd64.tar.gz";
        assert_eq!(have, want);
    }
}
