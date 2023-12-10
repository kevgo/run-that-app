use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::{download, filesystem, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// downloads an uncompressed precompiled binary
pub fn install(args: Args) -> Result<Option<Executable>> {
    let Some(artifact) = download::artifact(args.artifact_url, args.output)? else {
        return Ok(None);
    };
    if let Some(parent) = args.filepath_on_disk.parent() {
        fs::create_dir_all(parent).map_err(|err| UserError::CannotCreateFolder {
            folder: parent.to_path_buf(),
            reason: err.to_string(),
        })?;
    }
    let executable = filesystem::save_buffer(artifact.data, &args.filepath_on_disk, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct Args<'a> {
    pub artifact_url: String,
    pub filepath_on_disk: PathBuf,
    pub output: &'a dyn Output,
}
