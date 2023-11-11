use super::Artifact;
use crate::error::UserError;
use crate::Output;
use crate::Result;
use colored::Colorize;

pub fn http_get(url: String, output: &dyn Output) -> Result<Artifact> {
    output.print(&format!("downloading {} ... ", url.cyan()));
    let Ok(response) = minreq::get(&url).send() else {
        output.println(&format!("{}", "failed\n".red()));
        return Err(UserError::NotOnline);
    };
    if response.status_code == 404 {
        output.println(&format!("{}", "404".red()));
        return Err(UserError::UnsupportedPlatform);
    }
    if response.status_code != 200 {
        output.println(&format!("{}", "ERROR".red()));
        return Err(UserError::CannotDownload {
            reason: response.reason_phrase,
            url,
        });
    }
    output.println(&format!("{}", "ok".green()));
    Ok(Artifact {
        filename: url,
        data: response.into_bytes(),
    })
}
