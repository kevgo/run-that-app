use crate::output::Output;
use crate::yard::Executable;
use crate::{archives, filesystem};
use crate::{download, Result};
use colored::Colorize;
use std::path::PathBuf;

/// downloads and unpacks the entire content of an archive file
pub fn install(args: InstallArgs) -> Result<Option<Executable>> {
    let Some(artifact) = download::artifact(args.artifact_url, args.output)? else {
        return Ok(None);
    };
    filesystem::create_parent(&args.filepath_on_disk)?;
    let executable = archives::extract_all(artifact, &args.filepath_on_disk, "", args.executable_path_in_archive, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct InstallArgs<'a> {
    pub artifact_url: String,
    pub filepath_on_disk: PathBuf,
    pub executable_path_in_archive: &'a str,
    pub output: &'a dyn Output,
}
