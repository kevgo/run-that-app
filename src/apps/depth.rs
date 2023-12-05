use super::App;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::{download_executable, ArtifactType, DownloadArgs};
use crate::output::Output;
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::Result;

pub struct Depth {}

impl App for Depth {
    fn name(&self) -> &'static str {
        "depth"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "depth.exe",
            Os::Linux | Os::MacOS => "depth",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/KyleBanks/depth"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        if let Some(executable) = download_executable(&DownloadArgs {
            name: self.name(),
            url: download_url(version, platform),
            artifact_type: ArtifactType::Executable,
            file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            output,
        })? {
            return Ok(Some(executable));
        }
        compile_go(&CompileArgs {
            import_path: format!("github.com/KyleBanks/depth/cmd/depth@v{version}"),
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/KyleBanks/depth/releases/download/v{version}/depth_{version}_{os}_{cpu}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu)
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
        Cpu::Arm64 => "arm",
        Cpu::Intel64 => "amd64",
    }
}

#[cfg(test)]
mod tests {
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform {
            os: Os::Linux,
            cpu: Cpu::Intel64,
        };
        let have = super::download_url("1.2.1", platform);
        let want = "https://github.com/KyleBanks/depth/releases/download/v1.2.1/depth_1.2.1_linux_amd64";
        assert_eq!(have, want);
    }
}
