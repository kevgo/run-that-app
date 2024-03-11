use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::archive::{self, InstallArgs};
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Ghokin {}

const ORG: &str = "antham";
const REPO: &str = "ghokin";

impl App for Ghokin {
    fn name(&self) -> AppName {
        AppName::from("ghokin")
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    // fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
    //     let name = self.name();
    //     let result = archive::install(InstallArgs {
    //         app_name: &name,
    //         artifact_url: download_url(version, platform),
    //         output,
    //         dir_on_disk: yard.app_folder(&name, version),
    //         strip_path_prefix: "",
    //         executable_in_archive: &self.executable_filepath(platform),
    //     })?;
    //     if result.is_some() {
    //         return Ok(result);
    //     }
    //     compile_go(CompileArgs {
    //         import_path: format!("github.com/{ORG}/{REPO}/v3@v{version}"),
    //         target_folder: &yard.app_folder(&name, version),
    //         executable_filepath: self.executable_filepath(platform),
    //         output,
    //     })
    // }

    fn install_methods(&self) -> Vec<crate::install::Method> {
        vec![Method::DownloadArchive(self), Method::CompileGoSource(self)]
    }
    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        github_releases::versions("antham", "ghokin", amount, output)
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output("-h")) {
            return AnalyzeResult::NotIdentified;
        }
        // as of 3.4.0 ghokin's "version" command prints nothing
        AnalyzeResult::IdentifiedButUnknownVersion
    }
}

impl install::InstallByArchive for Ghokin {
    fn archive_url(&self, version: &Version, platform: Platform) -> String {
        format!(
            "https://github.com/{ORG}/{REPO}/releases/download/v{version}/ghokin_{version}_{os}_{cpu}.tar.gz",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
        )
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "amd64",
    }
}

fn identify(output: &str) -> bool {
    output.contains("Clean and/or apply transformation on gherkin files")
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
    mod download_url {
        use crate::config::Version;
        use crate::platform::{Cpu, Os, Platform};

        #[test]
        fn macos_intel64() {
            let platform = Platform { os: Os::MacOS, cpu: Cpu::Intel64 };
            let have = super::super::download_url(&Version::from("3.4.1"), platform);
            let want = "https://github.com/antham/ghokin/releases/download/v3.4.1/ghokin_3.4.1_darwin_amd64.tar.gz";
            assert_eq!(have, want);
        }

        #[test]
        fn windows_intel64() {
            let platform = Platform {
                os: Os::Windows,
                cpu: Cpu::Intel64,
            };
            let have = super::super::download_url(&Version::from("3.4.1"), platform);
            let want = "https://github.com/antham/ghokin/releases/download/v3.4.1/ghokin_3.4.1_windows_amd64.tar.gz";
            assert_eq!(have, want);
        }
    }
}
