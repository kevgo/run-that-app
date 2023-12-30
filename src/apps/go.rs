use super::App;
use crate::error::UserError;
use crate::hosting::github_tags;
use crate::install::archive::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use big_s::S;

pub struct Go {}

const ORG: &str = "golang";
const REPO: &str = "go";

impl App for Go {
    fn name(&self) -> &'static str {
        "go"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "go.exe",
            Os::Linux | Os::MacOS => "go",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://go.dev"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        // God object anti-pattern.
        // I call a complex subsystem with too many configuration parameters
        // and the subsystem does too many things including pulling in data it needs on its own.
        // This is hard to test.
        // Better approach is functional architecture:
        // step 1: download the artifact from the URL
        // step 2: extract the artifact while stripping paths
        // step 3: determine the executable
        archive::install(InstallArgs {
            app_name: self.name(),
            artifact_url: download_url(version, platform),
            dir_on_disk: yard.app_folder(self.name(), version),
            strip_path_prefix: "go/",
            executable_in_archive: &self.executable_path(platform),
            output,
        })
    }

    fn latest_version(&self, output: &dyn Output) -> Result<String> {
        let versions = self.versions(1, output)?;
        if versions.is_empty() {
            return Err(UserError::GitHubTagsApiProblem {
                problem: S("no tags found"),
                payload: S(""),
            });
        }
        Ok(versions.into_iter().next().unwrap())
    }

    fn load(&self, version: &str, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(self.name(), version, &self.executable_path(platform))
    }

    fn versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<String>> {
        let tags = github_tags::all(ORG, REPO, output)?;
        let mut go_tags: Vec<String> = tags.into_iter().filter(|tag| tag.starts_with("go")).collect();
        go_tags.sort_unstable_by(|a, b| human_sort::compare(b, a));
        if go_tags.len() > amount {
            go_tags.resize(amount, S(""));
        }
        Ok(go_tags)
    }
}

impl Go {
    fn executable_path(&self, platform: Platform) -> String {
        let executable = self.executable_filename(platform);
        match platform.os {
            Os::Windows => format!("bin\\{executable}"),
            Os::Linux | Os::MacOS => format!("bin/{executable}"),
        }
    }
}

pub fn download_url(version: &str, platform: Platform) -> String {
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

#[cfg(test)]
mod tests {
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform {
            os: Os::MacOS,
            cpu: Cpu::Arm64,
        };
        let have = super::download_url("1.21.5", platform);
        let want = "https://go.dev/dl/go1.21.5.darwin-arm64.tar.gz";
        assert_eq!(have, want);
    }
}
