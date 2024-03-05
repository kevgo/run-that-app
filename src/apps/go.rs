use std::num::FpCategory;

use super::App;
use crate::config::{AppName, Version};
use crate::hosting::github_tags;
use crate::install::archive::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::regex;
use crate::subshell::Executable;
use crate::yard::Yard;
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

    fn version(&self, executable: &Executable) -> Option<Version> {
        extract_version(&executable.run_output("version")).map(Version::from)
    }
}

pub fn download_url(version: &Version, platform: Platform) -> String {
    format!(
        "https://go.dev/dl/go{version}.{os}-{cpu}.{ext}",
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

fn extract_version(output: &str) -> Option<&str> {
    regex::first_capture(output, r"go(\d+\.\d+\.\d+)")
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
}
