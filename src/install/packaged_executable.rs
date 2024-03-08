use crate::config::AppName;
use crate::error::UserError;
use crate::output::Output;
use crate::subshell::Executable;
use crate::{archives, filesystem};
use crate::{download, Result};
use colored::Colorize;
use std::path::PathBuf;

/// downloads and installs a pre-compiled binary packaged in an archive file
pub fn install(args: InstallArgs) -> Result<Option<Executable>> {
    let Some(artifact) = download::artifact(args.artifact_url, args.app_name, args.output)? else {
        return Ok(None);
    };
    filesystem::create_parent(&args.filepath_on_disk)?;
    let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
        return Err(UserError::UnknownArchive(artifact.filename));
    };
    let executable = archive.extract_file(args.file_to_extract, &args.filepath_on_disk, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct InstallArgs<'a> {
    pub app_name: &'a AppName,
    pub artifact_url: String,
    pub file_to_extract: &'a str,
    pub filepath_on_disk: PathBuf,
    pub output: &'a dyn Output,
}
