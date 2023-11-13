use super::InstallationMethod;
use crate::output::Output;
use std::path::PathBuf;
use std::process::Command;

pub struct CompileFromGoSource {
    pub url: &'static str,
    pub version: String,
    pub target_folder: PathBuf,
}

impl InstallationMethod for CompileFromGoSource {
    fn install(&self, _output: &dyn Output) -> crate::Result<Option<crate::yard::Executable>> {
        let mut cmd = Command::new("go");
        cmd.arg("install");
        cmd.arg(format!(
            "{url}@{version}",
            url = self.url,
            version = self.version
        ));
        todo!()
    }
}
