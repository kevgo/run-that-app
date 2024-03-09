use crate::archives;
use crate::config::AppName;
use crate::output::Output;
use crate::UserError;
use crate::{download, Result};
use std::fs;
use std::path::Path;

/// downloads and unpacks the content of an archive file
pub fn install(args: InstallArgs) -> Result<bool> {
    let Some(artifact) = download::artifact(args.artifact_url, args.app_name, args.output)? else {
        return Ok(false);
    };
    fs::create_dir_all(&args.dir_on_disk).map_err(|err| UserError::CannotCreateFolder {
        folder: args.dir_on_disk.to_path_buf(),
        reason: err.to_string(),
    })?;
    let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
        return Err(UserError::UnknownArchive(artifact.filename));
    };
    archive.extract_all(&args.dir_on_disk, args.output)?;
    Ok(true)
}

pub struct InstallArgs<'a> {
    pub app_name: &'a AppName,
    pub artifact_url: String,
    pub dir_on_disk: &'a Path,
    pub executable_locations: Vec<String>,
    pub output: &'a dyn Output,
}
