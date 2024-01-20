use crate::output::Output;
use crate::subshell::Executable;
use crate::{download, filesystem, Result};
use colored::Colorize;
use std::path::PathBuf;

/// downloads an uncompressed precompiled binary
pub fn install(args: InstallArgs) -> Result<Option<Executable>> {
    let Some(artifact) = download::artifact(args.artifact_url, args.app_name, args.output)? else {
        return Ok(None);
    };
    filesystem::create_parent(&args.filepath_on_disk)?;
    let executable = filesystem::save_executable(artifact.data, &args.filepath_on_disk, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct InstallArgs<'a> {
    pub app_name: &'a str,
    pub artifact_url: String,
    pub filepath_on_disk: PathBuf,
    pub output: &'a dyn Output,
}
