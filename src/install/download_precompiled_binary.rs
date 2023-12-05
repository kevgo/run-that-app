use super::InstallationMethod;
use crate::archives;
use crate::download::http_get;
use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// downloads a pre-compiled binary from the internet
pub struct DownloadPrecompiledBinary {
    /// name of the app
    pub name: &'static str,

    /// URL of the artifact to download
    pub url: String,

    /// Some = name of the file to extract from the archive, None = the artifact is the file
    pub artifact_type: ArtifactType,

    /// path of the executable to create on disk
    pub file_on_disk: PathBuf,
}

pub enum ArtifactType {
    /// the downloaded artifact is the executable file we need
    Executable,
    /// the downloaded artifact is an archive file containing the executable file we need
    Archive { file_to_extract: String },
}

impl InstallationMethod for DownloadPrecompiledBinary {
    fn install(&self, output: &dyn Output) -> Result<Option<Executable>> {
        output.print("downloading ... ");
        output.log(CATEGORY, &format!("downloading {} ... ", url.cyan()));
        let Some(artifact) = http_get(self.url.clone(), output)? else {
            return Ok(None);
        };
        // create the folder here?
        if let Some(parent) = self.file_on_disk.parent() {
            fs::create_dir_all(parent).map_err(|err| UserError::CannotCreateFolder {
                folder: parent.to_path_buf(),
                reason: err.to_string(),
            })?;
        }
        let executable = archives::extract(artifact, &self.artifact_type, &self.file_on_disk, output)?;
        output.println(&format!("{}", "ok".green()));
        Ok(Some(executable))
    }
}
