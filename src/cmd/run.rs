use crate::cli::RunRequest;
use crate::{apps, detect, Output, Result};

pub fn run(request: RunRequest, output: &Output) -> Result<()> {
    let platform = detect::detect(output)?;
    let app = apps::lookup(&request.name)?;
    // install if needed
    let installed_app = yard::install_if_needed(&request)?;
    // execute the installed app
    Ok(())
}
