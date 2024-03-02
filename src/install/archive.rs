use crate::archives::{self, ExtractAllArgs};
use crate::config::AppName;
use crate::output::Output;
use crate::subshell::Executable;
use crate::UserError;
use crate::{download, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// downloads and unpacks the entire content of an archive file
pub fn install(args: InstallArgs) -> Result<Option<Executable>> {
    let Some(artifact) = download::artifact(args.artifact_url, args.app_name, args.output)? else {
        return Ok(None);
    };
    fs::create_dir_all(&args.dir_on_disk).map_err(|err| UserError::CannotCreateFolder {
        folder: args.dir_on_disk.clone(),
        reason: err.to_string(),
    })?;
    let executable = archives::extract_all(ExtractAllArgs {
        artifact,
        dir_on_disk: &args.dir_on_disk,
        strip_prefix: args.strip_path_prefix,
        executable_in_archive: args.executable_in_archive,
        output: args.output,
    })?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct InstallArgs<'a> {
    pub app_name: &'a AppName,
    pub artifact_url: String,
    pub dir_on_disk: PathBuf,
    pub strip_path_prefix: &'a str,
    pub executable_in_archive: &'a str,
    pub output: &'a dyn Output,
}
