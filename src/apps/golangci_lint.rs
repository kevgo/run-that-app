use super::{App, VersionResult};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::packaged_executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use const_format::formatcp;

pub struct GolangCiLint {}

const ORG: &str = "golangci";
const REPO: &str = "golangci-lint";

impl App for GolangCiLint {
    fn name(&self) -> AppName {
        AppName::from("golangci-lint")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "golangci-lint",
            Os::Windows => "golangci-lint.exe",
        }
    }

    fn executable_filepath(&self, platform: Platform) -> &'static str {
        self.executable_filename(platform)
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let name = self.name();
        packaged_executable::install(InstallArgs {
            app_name: &name,
            artifact_url: download_url(version, platform),
            file_to_extract: &executable_path(version, platform, self.executable_filepath(platform)),
            filepath_on_disk: yard.app_folder(&name, version).join(self.executable_filepath(platform)),
            output,
        })
        // install from source not recommended, see https://golangci-lint.run/usage/install/#install-from-source
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(&self.name(), version, self.executable_filepath(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, output)
    }

    fn version(&self, executable: &Executable) -> VersionResult {
        match extract_version(&executable.run_output("--version")) {
            Some(version) => VersionResult::IdentifiedWithVersion(version.into()),
            None => VersionResult::IdentifiedButUnknownVersion,
        }
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "amd64",
    }
}

fn download_url(version: &Version, platform: Platform) -> String {
    format!(
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/golangci-lint-{version}-{os}-{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

// TODO: move into the executable_filepath method of the App trait
fn executable_path(version: &Version, platform: Platform, filename: &str) -> String {
    format!("golangci-lint-{version}-{os}-{cpu}/{filename}", os = os_text(platform.os), cpu = cpu_text(platform.cpu),)
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux | Os::MacOS => "tar.gz",
        Os::Windows => "zip",
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"golangci-lint has version (\d+\.\d+\.\d+) built with")
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "darwin",
        Os::Windows => "windows",
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = super::download_url(&Version::from("1.55.2"), platform);
        let want = "https://github.com/golangci/golangci-lint/releases/download/v1.55.2/golangci-lint-1.55.2-darwin-arm64.tar.gz";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        assert_eq!(
            super::extract_version("golangci-lint has version 1.56.2 built with go1.22.0 from 58a724a0 on 2024-02-15T18:01:51Z"),
            Some("1.56.2")
        );
        assert_eq!(super::extract_version("other"), None);
    }
}
