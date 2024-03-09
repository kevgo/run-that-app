use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Depth {}

const ORG: &str = "KyleBanks";
const REPO: &str = "depth";

impl App for Depth {
    fn name(&self) -> AppName {
        AppName::from("depth")
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let name = self.name();
        let result = executable::install(InstallArgs {
            app_name: &name,
            artifact_url: download_url(version, platform),
            filepath_on_disk: yard.create_app_folder(&name, version)?.join(self.executable_filepath(platform)),
            output,
        })?;
        if result.is_some() {
            return Ok(result);
        }
        compile_go(CompileArgs {
            import_path: format!("github.com/{ORG}/{REPO}/cmd/depth@v{version}"),
            target_folder: &yard.app_folder(&name, version),
            executable_filepath: self.executable_filepath(platform),
            output,
        })
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(&self.name(), version, &self.executable_filepath(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output("-h")) {
            return AnalyzeResult::NotIdentified;
        }
        // as of 1.2.1 depth doesn't display the version of the installed executable
        AnalyzeResult::IdentifiedButUnknownVersion
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "aarch64", // the "arm" binaries don't run on Apple Silicon
        Cpu::Intel64 => "amd64",
    }
}

fn download_url(version: &Version, platform: Platform) -> String {
    format!(
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/depth_{version}_{os}_{cpu}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu)
    )
}

fn identify(output: &str) -> bool {
    output.contains("resolves dependencies of internal (stdlib) packages.")
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
        let platform = Platform { os: Os::Linux, cpu: Cpu::Intel64 };
        let have = super::download_url(&Version::from("1.2.1"), platform);
        let want = "https://github.com/KyleBanks/depth/releases/download/v1.2.1/depth_1.2.1_linux_amd64";
        assert_eq!(have, want);
    }
}
