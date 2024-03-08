use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::packaged_executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};

pub struct Goreleaser {}

const ORG: &str = "goreleaser";
const REPO: &str = "goreleaser";

impl App for Goreleaser {
    fn name(&self) -> AppName {
        AppName::from("goreleaser")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "goreleaser",
            Os::Windows => "goreleaser.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://goreleaser.com"
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let name = self.name();
        let result = packaged_executable::install(InstallArgs {
            app_name: &name,
            artifact_url: download_url(version, platform),
            file_to_extract: self.executable_filepath(platform),
            filepath_on_disk: yard.app_folder(&name, version).join(self.executable_filepath(platform)),
            output,
        })?;
        if result.is_some() {
            return Ok(result);
        }
        compile_go(CompileArgs {
            import_path: format!("github.com/{ORG}/{REPO}@{version}"),
            target_folder: &yard.app_folder(&name, version),
            executable_filepath: self.executable_filepath(platform),
            output,
        })
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

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        let output = &executable.run_output("-V");
        if !identify(output) {
            return AnalyzeResult::NotIdentified;
        }
        match extract_version(output) {
            Some(version) => AnalyzeResult::IdentifiedWithVersion(version.into()),
            None => AnalyzeResult::IdentifiedButUnknownVersion,
        }
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "x86_64",
    }
}

fn download_url(version: &Version, platform: Platform) -> String {
    format!(
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/goreleaser_{os}_{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux | Os::MacOS => "tar.gz",
        Os::Windows => "zip",
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"GitVersion:\s*(\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Deliver Go Binaries as fast and easily as possible")
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "Linux",
        Os::MacOS => "Darwin",
        Os::Windows => "Windows",
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = super::download_url(&Version::from("1.22.1"), platform);
        let want = "https://github.com/goreleaser/goreleaser/releases/download/v1.22.1/goreleaser_Darwin_arm64.tar.gz";
        assert_eq!(have, want);
    }

    mod extract_version {
        use super::super::extract_version;

        #[test]
        fn success() {
            let output = r"
  ____       ____      _
 / ___| ___ |  _ \ ___| | ___  __ _ ___  ___ _ __
| |  _ / _ \| |_) / _ \ |/ _ \/ _` / __|/ _ \ '__|
| |_| | (_) |  _ <  __/ |  __/ (_| \__ \  __/ |
 \____|\___/|_| \_\___|_|\___|\__,_|___/\___|_|
goreleaser: Deliver Go Binaries as fast and easily as possible
https://goreleaser.com

GitVersion:    1.24.0
GitCommit:     00c2ff733758f63201811c337f8d043e8fcc9d58
GitTreeState:  false
BuildDate:     2024-02-05T12:18:01Z
BuiltBy:       goreleaser
GoVersion:     go1.21.6
Compiler:      gc
ModuleSum:     h1:jsoS5T2CvPKOyECPATAo8hCvUaX8ok4iAq9m5Zyl1L0=
Platform:      linux/arm64
";
            assert_eq!(extract_version(output), Some("1.24.0"));
        }

        #[test]
        fn other() {
            assert_eq!(extract_version("other"), None);
        }
    }
}
