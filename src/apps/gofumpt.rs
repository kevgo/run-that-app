use super::{App, VersionResult};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Gofumpt {}

const ORG: &str = "mvdan";
const REPO: &str = "gofumpt";

impl App for Gofumpt {
    fn name(&self) -> AppName {
        AppName::from("gofumpt")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "gofumpt",
            Os::Windows => "gofumpt.exe",
        }
    }

    fn executable_filepath(&self, platform: Platform) -> &'static str {
        self.executable_filename(platform)
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let name = self.name();
        let result = executable::install(InstallArgs {
            app_name: &name,
            artifact_url: download_url(version, platform),
            filepath_on_disk: yard.app_folder(&name, version).join(self.executable_filepath(platform)),
            output,
        })?;
        if result.is_some() {
            return Ok(result);
        }
        compile_go(CompileArgs {
            import_path: format!("mvdan.cc/gofumpt@{version}"),
            target_folder: &yard.app_folder(&name, version),
            executable_filepath: self.executable_filepath(platform),
            output,
        })
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(&self.name(), version, self.executable_filepath(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, output)
    }

    fn version(&self, executable: &Executable) -> VersionResult {
        extract_version(&executable.run_output("--version")).map(Version::from)
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
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/gofumpt_v{version}_{os}_{cpu}{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Windows => ".exe",
        Os::Linux | Os::MacOS => "",
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"v(\d+\.\d+\.\d+) \(go")
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
        fn macos_arm64() {
            let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
            let have = super::super::download_url(&Version::from("0.5.0"), platform);
            let want = "https://github.com/mvdan/gofumpt/releases/download/v0.5.0/gofumpt_v0.5.0_darwin_arm64";
            assert_eq!(have, want);
        }

        #[test]
        fn windows_intel64() {
            let platform = Platform {
                os: Os::Windows,
                cpu: Cpu::Intel64,
            };
            let have = super::super::download_url(&Version::from("0.5.0"), platform);
            let want = "https://github.com/mvdan/gofumpt/releases/download/v0.5.0/gofumpt_v0.5.0_windows_amd64.exe";
            assert_eq!(have, want);
        }
    }

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("v0.6.0 (go1.21.6)"), Some("0.6.0"));
        assert_eq!(super::extract_version("other"), None);
    }
}
