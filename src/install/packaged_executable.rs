use crate::output::Output;
use crate::yard::Executable;
use crate::{archives, filesystem};
use crate::{download, Result};
use colored::Colorize;
use std::path::PathBuf;

/// downloads and installs a pre-compiled binary packaged in an archive file
pub fn install(args: &Args) -> Result<Option<Executable>> {
    let Some(artifact) = download::artifact(args.artifact_url.to_string(), args.output)? else {
        return Ok(None);
    };
    filesystem::create_parent(&args.filepath_on_disk)?;
    let executable = archives::extract_file(artifact, args.file_to_extract, &args.filepath_on_disk, args.output)?;
    args.output.println(&format!("{}", "ok".green()));
    Ok(Some(executable))
}

pub struct Args<'a> {
    pub artifact_url: String,
    pub file_to_extract: &'a str, // TODO: make this a &str
    pub filepath_on_disk: PathBuf,
    pub output: &'a dyn Output,
}
