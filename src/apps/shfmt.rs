use super::App;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::{download_executable, ArtifactType, DownloadArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};

pub struct Shfmt {}

impl App for Shfmt {
    fn name(&self) -> &'static str {
        "shfmt"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "shfmt.exe",
            Os::Linux | Os::MacOS => "shfmt",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/mvdan/sh"
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
            import_path: format!("mvdan.cc/sh/v3/cmd/shfmt@v{version}"),
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/mvdan/sh/releases/download/v{version}/shfmt_v{version}_{os}_{cpu}{ext}",
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
        Os::Linux | Os::MacOS => "",
        Os::Windows => ".exe",
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
        let have = super::download_url("3.7.0", platform);
        let want = "https://github.com/mvdan/sh/releases/download/v3.7.0/shfmt_v3.7.0_darwin_arm64";
        assert_eq!(have, want);
    }
}
