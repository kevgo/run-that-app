use super::{App, VersionResult};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::compile_rust::{compile_rust, CompileArgs};
use crate::install::packaged_executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use const_format::formatcp;

pub struct MdBook {}

const ORG: &str = "rust-lang";
const REPO: &str = "mdBook";

impl App for MdBook {
    fn name(&self) -> AppName {
        AppName::from("mdBook")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "mdbook",
            Os::Windows => "mdbook.exe",
        }
    }

    fn executable_filepath(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "bin/mdbook",
            Os::Windows => "bin//mdbook.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
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
        compile_rust(CompileArgs {
            crate_name: "mdbook",
            target_folder: yard.app_folder(&name, version),
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

    fn version(&self, executable: &Executable) -> VersionResult {
        if !identify(&executable.run_output("-h")) {
            return VersionResult::NotIdentified;
        }
        match extract_version(&executable.run_output("-V")) {
            Some(version) => VersionResult::IdentifiedWithVersion(version.into()),
            None => VersionResult::IdentifiedButUnknownVersion,
        }
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "aarch64",
        Cpu::Intel64 => "x86_64",
    }
}

fn download_url(version: &Version, platform: Platform) -> String {
    format!(
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/mdbook-v{version}-{cpu}-{os}.tar.gz",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu)
    )
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"mdbook v(\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Creates a book from markdown files")
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "unknown-linux-gnu",
        Os::MacOS => "apple-darwin",
        Os::Windows => "pc-windows-msvc",
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_intel() {
        let platform = Platform { os: Os::Linux, cpu: Cpu::Intel64 };
        let have = super::download_url(&Version::from("0.4.37"), platform);
        let want = "https://github.com/rust-lang/mdBook/releases/download/v0.4.37/mdbook-v0.4.37-x86_64-unknown-linux-gnu.tar.gz";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("mdbook v0.4.37"), Some("0.4.37"));
        assert_eq!(super::extract_version("other"), None);
    }
}
