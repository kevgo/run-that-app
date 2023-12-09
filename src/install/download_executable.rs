use crate::archives::{self, Artifact};
use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// downloads the uncompressed precompiled binary
pub fn download_executable(args: DownloadArgs) -> Result<Option<Executable>> {
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
    if let Some(parent) = args.filepath_on_disk.parent() {
        fs::create_dir_all(parent).map_err(|err| UserError::CannotCreateFolder {
            folder: parent.to_path_buf(),
            reason: err.to_string(),
        })?;
    }
    let artifact = Artifact {
        filename: args.artifact_url,
        data,
    };
    let executable = archives::store_executable(artifact, &args.filepath_on_disk, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct DownloadArgs<'a> {
    pub artifact_url: String,
    pub filepath_on_disk: PathBuf,
    pub output: &'a dyn Output,
}
