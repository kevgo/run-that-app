use crate::apps::App;
use crate::config::Version;
use crate::error::UserError;
use crate::{yard, Result};
use std::process::Command;
use which::which;

pub trait Data: App {
    fn crate_name(&self) -> &'static str;
}

/// installs the given Rust-based application by compiling it from source
pub fn run(app: &dyn Data, version: &Version) -> Result<bool> {
    let Ok(cargo_path) = which("cargo") else {
        return Err(UserError::RustNotInstalled);
    };
    let yard = yard::load_or_create(&yard::production_location()?)?;
    let target_folder = yard.app_folder(&app.yard_app(), version);
    let mut cmd = Command::new(cargo_path);
    cmd.arg("install");
    cmd.arg("--root");
    cmd.arg(&target_folder);
    cmd.arg("--locked");
    cmd.arg(app.crate_name());
    let status = match cmd.status() {
        Ok(status) => status,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => return Err(UserError::RustNotInstalled),
            std::io::ErrorKind::PermissionDenied => return Err(UserError::RustNoPermission),
            std::io::ErrorKind::Interrupted => return Ok(false),
            _ => panic!("{}", err.to_string()),
        },
    };
    if !status.success() {
        return Err(UserError::RustCompilationFailed);
    }
    Ok(true)
}
