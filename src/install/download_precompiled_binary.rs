use crate::archives::{self, Artifact};
use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// downloads a pre-compiled binary from the internet
pub fn download_precompiled_binary(args: DownloadPrecompiledBinary, output: &dyn Output) -> Result<Option<Executable>> {
    output.log("download/http", &format!("downloading {} ... ", args.url.cyan()));
    let Ok(response) = minreq::get(&args.url).send() else {
        output.println(&format!("{}", "not online".red()));
        return Err(UserError::NotOnline);
    };
    if response.status_code == 404 {
        output.println(&format!("{}", "not found".red()));
        return Ok(None);
    }
    if response.status_code != 200 {
        output.println(&format!("{}", response.status_code.to_string().red()));
        return Err(UserError::CannotDownload {
            reason: response.reason_phrase,
            url: args.url.to_string(),
        });
    }
    let data = response.into_bytes();
    // create the folder here?
    if let Some(parent) = args.file_on_disk.parent() {
        fs::create_dir_all(parent).map_err(|err| UserError::CannotCreateFolder {
            folder: parent.to_path_buf(),
            reason: err.to_string(),
        })?;
    }
    let artifact = Artifact {
        filename: args.url.clone(),
        data,
    };
    let executable = archives::extract(artifact, &args.artifact_type, &args.file_on_disk, output)?;
    output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

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
