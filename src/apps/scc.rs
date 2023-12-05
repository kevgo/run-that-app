use super::App;
use crate::install::{compile_go, download_executable, ArtifactType, CompileArgs, DownloadArgs};
use crate::output::Output;
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::Result;
use big_s::S;

pub struct Scc {}

impl App for Scc {
    fn name(&self) -> &'static str {
        "scc"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "scc.exe",
            Os::Linux | Os::MacOS => "scc",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/boyter/scc"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let download = DownloadArgs {
            name: self.name(),
            url: download_url(version, platform),
            artifact_type: ArtifactType::Archive {
                file_to_extract: S(self.executable_filename(platform)),
            },
            file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
        };
        if let Some(executable) = download_executable(download, output)? {
            return Ok(Some(Executable));
        }
        let compile = CompileArgs {
            import_path: format!("github.com/boyter/scc/v3@{version}"),
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
        };
        compile_go(compile, output)
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/boyter/scc/releases/download/v{version}/scc_{version}_{os}_{cpu}.{ext}",
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
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform {
            os: Os::MacOS,
            cpu: Cpu::Arm64,
        };
        let have = super::download_url("3.1.0", platform);
        let want = "https://github.com/boyter/scc/releases/download/v3.1.0/scc_3.1.0_Darwin_arm64.tar.gz";
        assert_eq!(have, want);
    }
}
