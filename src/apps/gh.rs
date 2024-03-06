use super::{App, VersionResult};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::packaged_executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};

pub struct Gh {}

const ORG: &str = "cli";
const REPO: &str = "cli";

impl App for Gh {
    fn name(&self) -> AppName {
        AppName::from("gh")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "gh",
            Os::Windows => "gh.exe",
        }
    }

    fn executable_filepath(&self, platform: Platform) -> &'static str {
        self.executable_filename(platform)
    }

    fn homepage(&self) -> &'static str {
        "https://cli.github.com"
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let name = self.name();
        packaged_executable::install(InstallArgs {
            app_name: &name,
            artifact_url: download_url(version, platform),
            file_to_extract: &executable_path(version, platform),
            filepath_on_disk: yard.app_folder(&name, version).join(self.executable_filepath(platform)),
            output,
        })
        // installation from source seems more involved, see https://github.com/cli/cli/blob/trunk/docs/source.md
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
        if !identify(&executable.run_output("-h")) {
            return VersionResult::NotIdentified;
        }
        match extract_version(&executable.run_output("--version")) {
            Some(version) => VersionResult::IdentifiedWithVersion(Version::from(version)),
            None => VersionResult::IdentifiedButUnknownVersion,
        }
    }
}

fn download_url(version: &Version, platform: Platform) -> String {
    format!(
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/gh_{version}_{os}_{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"gh version (\d+\.\d+\.\d+)")
}

fn executable_path(version: &Version, platform: Platform) -> String {
    match platform.os {
        Os::Windows => "bin/gh.exe".to_string(),
        Os::Linux | Os::MacOS => format!("gh_{version}_{os}_{cpu}/bin/gh", os = os_text(platform.os), cpu = cpu_text(platform.cpu)),
    }
}

fn identify(output: &str) -> bool {
    output.contains("Work seamlessly with GitHub from the command line")
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
    use crate::config::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform { os: Os::Linux, cpu: Cpu::Intel64 };
        let have = super::download_url(&Version::from("2.39.1"), platform);
        let want = "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_linux_amd64.tar.gz";
        assert_eq!(have, want);
    }

    mod extract_version {
        use super::super::extract_version;

        #[test]
        fn success() {
            let output = "
gh version 2.45.0 (2024-03-04)
https://github.com/cli/cli/releases/tag/v2.45.0
";
            assert_eq!(extract_version(output), Some("2.45.0"));
        }

        #[test]
        fn other() {
            assert_eq!(extract_version("other"), None);
        }
    }
}
