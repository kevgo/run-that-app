use super::InstallationMethod;
use crate::archives;
use crate::download::http_get;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use std::path::PathBuf;

/// downloads a pre-compiled binary from the internet
pub struct DownloadPrecompiledBinary {
    /// URL of the artifact to download
    pub url: String,

    /// Some = name of the file to extract from the archive, None = the artifact is the file
    pub file_in_archive: Option<String>,

    /// path of the executable to create on disk
    pub file_on_disk: PathBuf,
}

impl InstallationMethod for DownloadPrecompiledBinary {
    fn install(&self, output: &dyn Output) -> Result<Option<Executable>> {
        let Some(artifact) = http_get(self.url.clone(), output)? else {
            return Ok(None);
        };
        let executable =
            archives::extract(artifact, &self.file_in_archive, &self.file_on_disk, output)?;
        Ok(Some(executable))
    }
}
