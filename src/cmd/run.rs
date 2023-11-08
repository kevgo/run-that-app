use crate::cli::RunRequest;
use crate::{apps, detect, Output, Result};

pub fn run(request: RunRequest, output: &Output) -> Result<()> {
    let platform = detect::detect(output)?;
    let app = apps::lookup(&request.name)?;
    // install if needed
    // execute the installed app
    Ok(())
}
