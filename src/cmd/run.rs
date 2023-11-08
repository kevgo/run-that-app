use crate::cli::RunRequest;
use crate::{platform, Output, Result};

pub fn run(app: RunRequest, output: &Output) -> Result<()> {
    let platform = platform::detect(output)?;
    Ok(())
}
