use crate::output::{Event, Output};
use crate::subshell::Executable;
use which::which_global;

pub fn find_global_install(binary_name: &str, output: Output) -> Option<Executable> {
    output(Event::GlobalInstallSearch { binary: binary_name });
    if let Ok(path) = which_global(binary_name) {
        output(Event::GlobalInstallFound { path: &path });
        Some(Executable(path))
    } else {
        output(Event::GlobalInstallNotFound);
        None
    }
}
