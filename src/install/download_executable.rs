use crate::archives::{self, Artifact};
use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// installs the given application by downloading its pre-compiled binary
pub fn download_executable(args: &DownloadArgs) -> Result<Option<Executable>> {
    if args.output.is_active("download") {
        args.output.log("download", &format!("downloading {} ... ", args.url.cyan()));
    } else {
        args.output.print("downloading ... ")
    }
    let Ok(response) = minreq::get(&args.url).send() else {
        args.output.println(&format!("{}", "not online".red()));
        return Err(UserError::NotOnline);
    };
    if response.status_code == 404 {
        args.output.println(&format!("{}", "not found".red()));
        return Ok(None);
    }
    if response.status_code != 200 {
        args.output.println(&format!("{}", response.status_code.to_string().red()));
        return Err(UserError::CannotDownload {
            reason: response.reason_phrase,
            url: args.url.to_string(),
        });
    }
    let data = response.into_bytes();
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
    let executable = archives::extract(artifact, &args.artifact_type, &args.file_on_disk, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct DownloadArgs<'a> {
    /// name of the app
    pub name: &'static str,

    /// URL of the artifact to download
    pub url: String,

    /// Some = name of the file to extract from the archive, None = the artifact is the file
    pub artifact_type: ArtifactType,

    /// path of the executable to create on disk
    pub file_on_disk: PathBuf,

    pub output: &'a dyn Output,
}

pub enum ArtifactType {
    /// the downloaded artifact is the executable file we need
    Executable,
    /// the downloaded artifact is an archive file containing the executable file we need
    Archive { file_to_extract: String },
}
