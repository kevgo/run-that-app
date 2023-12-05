use super::App;
use crate::install::{download_executable, ArtifactType, DownloadArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};

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

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        download_executable(&DownloadArgs {
            app_name: self.name(),
            artifact_url: download_url(version, platform),
            artifact_type: ArtifactType::Archive {
                file_to_extract: executable_path(version, platform),
            },
            file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            output,
        })
        // installation from source seems more involved, see https://github.com/cli/cli/blob/trunk/docs/source.md
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
