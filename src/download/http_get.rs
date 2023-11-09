use super::Artifact;
use crate::error::UserError;
use crate::Output;
use crate::Result;
use colored::Colorize;

pub fn http_get(url: String, output: &dyn Output) -> Result<Artifact> {
    output.print(&format!("downloading {} ... ", url.cyan()));
    let Ok(response) = minreq::get(&url).send() else {
        println!("{}", "failed\n".red());
        return Err(UserError::NotOnline);
    };
    if response.status_code == 404 {
        println!("{}", "404".red());
        return Err(UserError::UnsupportedPlatform);
    }
    if response.status_code != 200 {
        println!("{}", "ERROR".red());
        return Err(UserError::CannotDownload {
            reason: response.reason_phrase,
            url: url.to_string(),
        });
    }
    output.println(&format!("{}", "ok".green()));
    Ok(Artifact {
        filename: url,
        data: response.into_bytes(),
    })
}
