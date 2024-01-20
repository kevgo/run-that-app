use crate::output::Output;
use crate::subshell::Executable;
use std::path::Path;
use which::which_global;

pub fn find_global_install(binary_name: &str, output: &dyn Output) -> Option<Executable> {
    if let Ok(path) = which_global(binary_name) {
        log(output, &path);
        Some(Executable(path))
    } else {
        None
    }
}

fn log(output: &dyn Output, path: &Path) {
    output.log(CATEGORY, &format!("using globally installed {}", path.to_string_lossy()));
}

const CATEGORY: &str = "detect/executable";
