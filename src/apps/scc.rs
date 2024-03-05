use super::App;
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::packaged_executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Scc {}

const ORG: &str = "boyter";
const REPO: &str = "scc";

impl App for Scc {
    fn name(&self) -> AppName {
        AppName::from("scc")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "scc",
            Os::Windows => "scc.exe",
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
            import_path: format!("github.com/{ORG}/{REPO}/v3@v{version}"),
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
}

fn download_url(version: &Version, platform: Platform) -> String {
    format!(
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/scc_{version}_{os}_{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "Linux",
        Os::MacOS => "Darwin",
        Os::Windows => "Windows",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "x86_64",
    }
}

fn ext_text(_os: Os) -> &'static str {
    "tar.gz"
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = super::download_url(&Version::from("3.1.0"), platform);
        let want = "https://github.com/boyter/scc/releases/download/v3.1.0/scc_3.1.0_Darwin_arm64.tar.gz";
        assert_eq!(have, want);
    }
}
