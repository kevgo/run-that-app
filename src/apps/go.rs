use super::App;
use crate::install::archive::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use big_s::S;

pub struct Go {}

impl App for Go {
    fn name(&self) -> &'static str {
        "Go"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "go.exe",
            Os::Linux | Os::MacOS => "go",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://golang.org"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        archive::install(InstallArgs {
            artifact_url: download_url(version, platform),
            target_dir: yard.app_folder(self.name(), version),
            strip_prefix: "",
            executable_path_in_archive: executable_path(platform),
            output,
        })
    }

    fn latest_version(&self, _output: &dyn Output) -> Result<String> {
        Ok(S("1.21.5"))
        // TODO: parse https://go.dev/dl/?mode=json (which only has the most recent 2 versions)
        // or clone the Go repo and look at the tags (git clone https://go.googlesource.com/go).
    }

    fn load(&self, version: &str, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(self.name(), version, self.executable_filename(platform))
    }

    fn versions(&self, _amount: u8, _output: &dyn Output) -> Result<Vec<String>> {
        // TODO: clone the Go repo and look at the tags (git clone https://go.googlesource.com/go)
        Ok(vec![S("1.21.5")])
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

fn executable_path(platform: Platform) -> &'static str {
    match platform.os {
        Os::Windows => "bin\\node.exe",
        Os::Linux | Os::MacOS => "bin/node",
    }
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
