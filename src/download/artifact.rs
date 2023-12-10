use crate::output::Output;
use crate::{Result, UserError};
use colored::Colorize;

/// downloads the artifact at the given URL
pub fn artifact(url: String, output: &dyn Output) -> Result<Option<Artifact>> {
    if output.is_active("download") {
        output.print(&format!("downloading {} ... ", url.cyan()));
    } else {
        output.print("downloading ... ");
    }
    let Ok(response) = minreq::get(&url).send() else {
        output.println(&format!("{}", "not online".red()));
        return Err(UserError::NotOnline);
    };
    if response.status_code == 404 {
        output.println(&format!("{}", "not found".red()));
        return Ok(None);
    }
    if response.status_code != 200 {
        output.println(&format!("{}", response.status_code.to_string().red()));
        return Err(UserError::CannotDownload {
            reason: response.reason_phrase,
            url: url.to_string(),
        });
    }
    Ok(Some(Artifact {
        filename: url,
        data: response.into_bytes(),
    }))
}

/// An artifacts is a file containing an application, downloaded from the internet.
/// An artifact could be an archive containing the application binary (and other files),
/// or the uncompressed application binary itself.
pub struct Artifact {
    pub filename: String,
    pub data: Vec<u8>,
}
