use super::{App, ExecutableIdentity};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::archive::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};

pub struct NodeJS {}

pub const ORG: &str = "nodejs";
pub const REPO: &str = "node";

impl App for NodeJS {
    fn name(&self) -> AppName {
        AppName::from("node")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "node",
            Os::Windows => "node.exe",
        }
    }

    fn executable_filepath(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "bin\\node.exe",
            Os::Linux | Os::MacOS => "bin/node",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://nodejs.org"
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let name = self.name();
        archive::install(InstallArgs {
            app_name: &name,
            artifact_url: download_url(version, platform),
            dir_on_disk: yard.app_folder(&name, version),
            strip_path_prefix: &format!("node-v{version}-{os}-{cpu}/", os = os_text(platform.os), cpu = cpu_text(platform.cpu)),
            executable_in_archive: self.executable_filepath(platform),
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

    fn identify_executable(&self, executable: &Executable) -> ExecutableIdentity {
        if !identify(&executable.run_output("-h")) {
            return ExecutableIdentity::NotIdentified;
        }
        match extract_version(&executable.run_output("--version")) {
            Some(version) => ExecutableIdentity::IdentifiedWithVersion(version.into()),
            None => ExecutableIdentity::IdentifiedButUnknownVersion,
        }
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "x64",
    }
}

fn download_url(version: &Version, platform: Platform) -> String {
    format!(
        "https://nodejs.org/dist/v{version}/node-v{version}-{os}-{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "tar.xz",
        Os::MacOS => "tar.gz",
        Os::Windows => "zip",
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"v(\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Documentation can be found at https://nodejs.org")
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "darwin",
        Os::Windows => "win",
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = super::download_url(&Version::from("20.10.0"), platform);
        let want = "https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-arm64.tar.gz";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("v10.2.4"), Some("10.2.4"));
        assert_eq!(super::extract_version("other"), None);
    }
}
