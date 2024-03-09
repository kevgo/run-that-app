use crate::config::AppName;
use crate::output::Output;
use crate::{download, filesystem, Result};
use colored::Colorize;
use std::path::Path;

/// downloads an uncompressed precompiled binary
pub fn install(args: InstallArgs) -> Result<bool> {
    let Some(artifact) = download::artifact(args.artifact_url, args.app_name, args.output)? else {
        return Ok(false);
    };
    filesystem::create_parent(&args.filepath_on_disk)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(true)
}

pub struct InstallArgs<'a> {
    pub app_name: &'a AppName,
    pub artifact_url: String,
    pub filepath_on_disk: &'a Path,
    pub output: &'a dyn Output,
}
