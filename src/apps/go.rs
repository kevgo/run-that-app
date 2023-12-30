use super::App;
use crate::hosting::github_tags;
use crate::install::archive::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use const_format::formatcp;

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
        formatcp!("https://go.dev")
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
        github_tags::latest(ORG, REPO, output)
    }

    fn load(&self, version: &str, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(self.name(), version, &self.executable_path(platform))
    }

    fn versions(&self, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
        // The current implementation is too magical.
        // It calls a big complex subsystem with too many parameters
        // and it does too many things, including reaching out and pulling in needed data.
        // This makes the code hard to test.
        // Better to use a functional architecture.
        // step 1: download the JSON from the GitHub API
        // step 2: parse the JSON into tags - this is now testable with app-specific data
        // step 3: filter out non-go tags - this is now testable with app-specific data
        // step 4: sort
        // step 5: limit to the given amount
        let mut filtered: Vec<String> = github_tags::versions(ORG, REPO, amount, output)?
            .into_iter()
            .filter(|tag| tag.starts_with("go"))
            .collect();
        filtered.sort_unstable_by(|a, b| b.cmp(a));
        Ok(filtered)
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
