use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{regexp, Output, Result};
use const_format::formatcp;

pub struct Shfmt {}

const ORG: &str = "mvdan";
const REPO: &str = "sh";

impl App for Shfmt {
    fn name(&self) -> AppName {
        AppName::from("shfmt")
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
            import_path: format!("mvdan.cc/sh/v3/cmd/shfmt@v{version}"),
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
        match extract_version(&executable.run_output("--version")) {
            Some(version) => AnalyzeResult::IdentifiedWithVersion(version.into()),
            None => AnalyzeResult::IdentifiedButUnknownVersion,
        }
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
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/shfmt_v{version}_{os}_{cpu}{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux | Os::MacOS => "",
        Os::Windows => ".exe",
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"^v(\d+\.\d+\.\d+)$")
}

fn identify(output: &str) -> bool {
    output.contains("shfmt formats shell programs")
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
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = super::download_url(&Version::from("3.7.0"), platform);
        let want = "https://github.com/mvdan/sh/releases/download/v3.7.0/shfmt_v3.7.0_darwin_arm64";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("v3.7.0"), Some("3.7.0"));
        assert_eq!(super::extract_version("3.7.0"), None);
        assert_eq!(super::extract_version("other"), None);
    }
}
