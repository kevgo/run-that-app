use crate::apps::App;
use crate::config::AppName;
use crate::output::Output;
use crate::{download, filesystem, Result};
use colored::Colorize;
use std::path::Path;

pub trait DownloadExecutable: App {
    fn artifact_url(&self) -> String;
}

/// downloads an uncompressed precompiled binary
pub fn install(app: &dyn DownloadExecutable, output: &dyn Output) -> Result<bool> {
    let Some(artifact) = download::artifact(app.artifact_url(), &app.name(), output)? else {
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
