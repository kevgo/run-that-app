use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use crate::{archives, download};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// downloads and extracts the given application by archive
pub fn download_archive(args: &DownloadArgs) -> Result<Option<Executable>> {
    let Some(artifact) = download::artifact(args.artifact_url, args.output)? else {
        return Ok(None);
    };
    fs::create_dir_all(args.folder_on_disk).map_err(|err| UserError::CannotCreateFolder {
        folder: args.folder_on_disk,
        reason: err.to_string(),
    })?;
    let executable = archives::extract(artifact, &args.folder_on_disk, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct DownloadArgs<'a> {
    pub app_name: &'static str,
    pub artifact_url: String,
    pub folder_on_disk: PathBuf,
    pub output: &'a dyn Output,
}
