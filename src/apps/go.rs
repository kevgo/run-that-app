use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::error::UserError;
use crate::hosting::github_tags;
use crate::install::archive::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{filesystem, regexp};
use crate::{Output, Result};
use big_s::S;

pub struct Go {}

const ORG: &str = "golang";
const REPO: &str = "go";

impl App for Go {
    fn name(&self) -> AppName {
        AppName::from("go")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "go",
            Os::Windows => "go.exe",
        }
    }

    fn executable_filepath(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "bin/go",
            Os::Windows => "bin\\go.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://go.dev"
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let name = self.name();
        archive::install(InstallArgs {
            app_name: &name,
            artifact_url: download_url(version, platform),
            dir_on_disk: yard.app_folder(&name, version),
            strip_path_prefix: "go/",
            executable_in_archive: self.executable_filepath(platform),
            output,
        })
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        let versions = self.installable_versions(1, output)?;
        Ok(versions.into_iter().next().unwrap())
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(&self.name(), version, self.executable_filepath(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        let tags = github_tags::all(ORG, REPO, 100, output)?;
        let mut go_tags: Vec<String> = tags.into_iter().filter(|tag| tag.starts_with("go")).filter(|tag| !tag.contains("rc")).collect();
        go_tags.sort_unstable_by(|a, b| human_sort::compare(b, a));
        if go_tags.len() > amount {
            go_tags.resize(amount, S(""));
        }
        Ok(go_tags.into_iter().map(Version::from).collect())
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if let Some(version) = extract_version(&executable.run_output("version")) {
            return AnalyzeResult::IdentifiedWithVersion(version.into());
        }
        if identify(&executable.run_output("-h")) {
            AnalyzeResult::IdentifiedButUnknownVersion
        } else {
            AnalyzeResult::NotIdentified
        }
    }

    fn allowed_versions(&self) -> Result<Option<semver::VersionReq>> {
        let Some(go_mod_content) = filesystem::read_file("go.mod")? else {
            return Ok(None);
        };
        let Some(go_version_req) = parse_go_mod(&go_mod_content) else {
            return Ok(None);
        };
        let version_req = semver::VersionReq::parse(go_version_req).map_err(|err| UserError::CannotParseSemverRange {
            expression: go_version_req.to_string(),
            reason: err.to_string(),
        })?;
        Ok(Some(version_req))
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
        "https://go.dev/dl/go{version}.{os}-{cpu}.{ext}",
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
    regexp::first_capture(output, r"go version go(\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Go is a tool for managing Go source code")
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "darwin",
        Os::Windows => "windows",
    }
}

fn parse_go_mod(text: &str) -> Option<&str> {
    regexp::first_capture(text, r"(?m)^go\s+(\d+\.\d+)\s*$")
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = super::download_url(&Version::from("1.21.5"), platform);
        let want = "https://go.dev/dl/go1.21.5.darwin-arm64.tar.gz";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        let give = "go version go1.21.7 linux/arm64";
        let have = super::extract_version(give);
        let want = Some("1.21.7");
        assert_eq!(have, want);
    }

    mod parse_go_mod {
        use crate::apps::go::parse_go_mod;

        #[test]
        fn with_version() {
            let go_mod = "
module github.com/git-town/git-town/v12

go 1.21

require (
	code.gitea.io/sdk/gitea v0.17.1
	github.com/BurntSushi/toml v1.3.2
	github.com/acarl005/stripansi v0.0.0-20180116102854-5a71ef0e047d
	github.com/charmbracelet/bubbles v0.18.0
	github.com/charmbracelet/bubbletea v0.25.0
)";
            assert_eq!(parse_go_mod(go_mod), Some("1.21"));
        }

        #[test]
        fn without_version() {
            let go_mod = "
module github.com/git-town/git-town/v12

require (
	code.gitea.io/sdk/gitea v0.17.1
	github.com/BurntSushi/toml v1.3.2
	github.com/acarl005/stripansi v0.0.0-20180116102854-5a71ef0e047d
	github.com/charmbracelet/bubbles v0.18.0
	github.com/charmbracelet/bubbletea v0.25.0
)";
            assert_eq!(parse_go_mod(go_mod), None);
        }

        #[test]
        fn unrelated_file() {
            let go_mod = "other file randomly named go.mod";
            assert_eq!(parse_go_mod(go_mod), None);
        }
    }
}
