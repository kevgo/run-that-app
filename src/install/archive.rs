use crate::archives::{self, ExtractAllArgs, ExtractDirArgs};
use crate::output::Output;
use crate::yard::Executable;
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
    pub app_name: &'a str,
    pub artifact_url: String,
    pub target_dir: PathBuf,
    pub strip_prefix: &'a str,
    pub executable_path_in_archive: &'a str,
    pub output: &'a dyn Output,
}

/// downloads and unpacks the entire content of an archive file
pub fn install_subdir(args: InstallDirArgs) -> Result<Option<Executable>> {
    let Some(artifact) = download::artifact(args.artifact_url, args.app_name, args.output)? else {
        return Ok(None);
    };
    fs::create_dir_all(&args.dir_on_disk).map_err(|err| UserError::CannotCreateFolder {
        folder: args.dir_on_disk.clone(),
        reason: err.to_string(),
    })?;
    let executable = archives::extract_dir(ExtractDirArgs {
        artifact,
        dir_in_archive: args.dir_in_archive,
        dir_on_disk: &args.dir_on_disk,
        strip_prefix: args.strip_prefix,
        executable_in_archive: args.executable_in_archive,
        output: args.output,
    })?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct InstallDirArgs<'a> {
    pub app_name: &'a str,
    pub artifact_url: String,
    pub dir_in_archive: &'a str,
    pub dir_on_disk: PathBuf,
    pub strip_prefix: &'a str,
    pub executable_in_archive: &'a str,
    pub output: &'a dyn Output,
}
