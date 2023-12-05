use crate::error::UserError;
use crate::Output;
use crate::Result;
use colored::Colorize;

/// downloads data at the given URL,
/// indicates 404 with None
pub fn http_get(url: &str, output: &dyn Output) -> Result<Option<Vec<u8>>> {
    output.log(CATEGORY, &format!("downloading {} ... ", url.cyan()));
    let Ok(response) = minreq::get(url).send() else {
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
    Ok(Some(response.into_bytes()))
}

const CATEGORY: &str = "download/http";
