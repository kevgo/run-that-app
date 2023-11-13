use super::Artifact;
use crate::error::UserError;
use crate::Output;
use crate::Result;
use colored::Colorize;

pub fn http_get(url: String, output: &dyn Output) -> Result<Option<Artifact>> {
    output.print(&format!("downloading {} ... ", url.cyan()));
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
            url,
        });
    }
    output.println(&format!("{}", "ok".green()));
    Ok(Some(Artifact {
        filename: url,
        data: response.into_bytes(),
    }))
}
