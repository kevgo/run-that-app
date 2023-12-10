use crate::error::UserError;
use crate::output::Output;
use crate::Result;
use crate::{archives, download};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// downloads and extracts the given application by archive
pub fn install(args: Args) -> Result<Option<()>> {
    let Some(artifact) = download::artifact(args.artifact_url, args.output)? else {
        return Ok(None);
    };
    fs::create_dir_all(&args.folder_on_disk).map_err(|err| UserError::CannotCreateFolder {
        folder: args.folder_on_disk.clone(),
        reason: err.to_string(),
    })?;
    archives::extract_all(artifact, &args.folder_on_disk, args.trim, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(()))
}

pub struct Args<'a> {
    pub artifact_url: String,
    pub folder_on_disk: PathBuf,
    pub trim: &'a str,
    pub output: &'a dyn Output,
}
