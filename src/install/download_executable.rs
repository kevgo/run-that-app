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
        args.output.print(&format!("downloading {} ... ", args.artifact_url.cyan()));
    } else {
        args.output.print("downloading ... ");
    }
    let Ok(response) = minreq::get(&args.artifact_url).send() else {
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
            url: args.artifact_url.to_string(),
        });
    }
    let data = response.into_bytes();
    fs::create_dir_all(&args.disk_folder).map_err(|err| UserError::CannotCreateFolder {
        folder: args.disk_folder.clone(),
        reason: err.to_string(),
    })?;
    let artifact = Artifact {
        filename: args.artifact_url.clone(),
        data,
    };
    let executable = archives::extract(artifact, &args.artifact_type, &args.disk_folder, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct DownloadArgs<'a> {
    pub app_name: &'static str,
    pub artifact_url: String,
    pub artifact_type: ArtifactType,
    pub disk_folder: PathBuf,
    pub output: &'a dyn Output,
}

pub enum ArtifactType {
    /// the executable can be downloaded directly
    Executable,
    /// the executable is packaged inside a zip or tar.gz file
    PackagedExecutable { file_to_extract: String },
}
