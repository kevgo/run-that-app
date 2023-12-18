use crate::archives::{self, ExtractAllArgs};
use crate::output::Output;
use crate::yard::Executable;
use crate::UserError;
use crate::{download, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// downloads and unpacks the entire content of an archive file
pub fn install(args: InstallArgs) -> Result<Option<Executable>> {
    let Some(artifact) = download::artifact(args.artifact_url, args.output)? else {
        return Ok(None);
    };
    fs::create_dir_all(&args.target_dir).map_err(|err| UserError::CannotCreateFolder {
        folder: args.target_dir.clone(),
        reason: err.to_string(),
    })?;
    let executable = archives::extract_all(ExtractAllArgs {
        artifact,
        target_dir: &args.target_dir,
        strip_prefix: args.strip_prefix,
        executable_path_in_archive: args.executable_path_in_archive,
        output: args.output,
    })?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct InstallArgs<'a> {
    pub artifact_url: String,
    pub target_dir: PathBuf,
    pub strip_prefix: &'a str,
    pub executable_path_in_archive: &'a str,
    pub output: &'a dyn Output,
}
