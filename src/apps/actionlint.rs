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

pub struct ActionLint {}

const ORG: &str = "rhysd";
const REPO: &str = "actionlint";

impl App for ActionLint {
    fn name(&self) -> AppName {
        AppName::from("actionlint")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "actionlint",
            Os::Windows => "actionlint.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://{ORG}.github.io/{REPO}")
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let name = self.name();
        let result = packaged_executable::install(InstallArgs {
            app_name: &name,
            artifact_url: download_url(version, platform),
            file_to_extract: self.executable_filename(platform),
            filepath_on_disk: yard.app_folder(&name, version).join(self.executable_filename(platform)),
            output,
        })?;
        if result.is_some() {
            return Ok(result);
        }
        compile_go(CompileArgs {
            import_path: format!("github.com/{ORG}/{REPO}/cmd/actionlint@{version}"),
            target_folder: &yard.app_folder(&name, version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(&self.name(), version, self.executable_filename(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, output)
    }

    fn version(&self, path: &Executable) -> Option<String> {
        todo!()
    }
}

fn download_url(version: &Version, platform: Platform) -> String {
    format!(
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/actionlint_{version}_{os}_{cpu}.{ext}",
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

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform { os: Os::Linux, cpu: Cpu::Arm64 };
        let have = super::download_url(&Version::from("1.6.26"), platform);
        let want = "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_linux_arm64.tar.gz";
        assert_eq!(have, want);
    }
}
