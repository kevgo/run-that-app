use crate::output::Output;
use std::path::{Path, PathBuf};
use which::which_global;

pub fn find_global_install(binary_name: &str, output: &dyn Output) -> Option<PathBuf> {
    if let Ok(path) = which_global(binary_name) {
        log(output, &path);
        Some(path)
    } else {
        None
    }
}

fn log(output: &dyn Output, path: &Path) {
    output.log(
        CATEGORY,
        &format!("using globally installed {}", path.to_string_lossy()),
    );
}

const CATEGORY: &str = "detect/executable";
