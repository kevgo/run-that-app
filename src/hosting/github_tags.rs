use crate::{Output, Result, UserError};
use colored::Colorize;
use miniserde::{json, Deserialize};

/// provides the latest version that the given application is tagged with on GitHub
pub fn latest(org: &str, repo: &str, output: &dyn Output) -> Result<String> {
    let url = format!("https://api.github.com/repos/{org}/{repo}/git/refs/tags");
    output.log("HTTP", &format!("downloading {url}"));
    let get = minreq::get(&url)
        .with_header("Accept", "application/vnd.github+json")
        .with_header("User-Agent", format!("run-that-app-{}", env!("CARGO_PKG_VERSION")))
        .with_header("X-GitHub-Api-Version", "2022-11-28");
    let Ok(response) = get.send() else {
        output.println(&format!("{}", "not online".red()));
        return Err(UserError::NotOnline);
    };
    let response_text = response.as_str().unwrap();
    let release: Tag = match json::from_str(response_text) {
        Ok(release) => release,
        Err(err) => {
            println!("{}", "Error:".red());
            println!("\n{response_text}");
            return Err(UserError::CannotDownload { url, reason: err.to_string() });
        }
    };
    Ok(release.standardized_tag().to_string())
}

pub fn versions(org: &str, repo: &str, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
    let url = format!("https://api.github.com/repos/{org}/{repo}/git/refs/tags?per_page={amount}");
    output.log("HTTP", &format!("downloading {url}"));
    let get = minreq::get(&url)
        .with_param("per_page", amount.to_string())
        .with_header("Accept", "application/vnd.github+json")
        .with_header("User-Agent", format!("run-that-app-{}", env!("CARGO_PKG_VERSION")))
        .with_header("X-GitHub-Api-Version", "2022-11-28");
    let Ok(response) = get.send() else {
        output.println(&format!("{}", "not online".red()));
        return Err(UserError::NotOnline);
    };
    let response_text = response.as_str().unwrap();
    let tags: Vec<Tag> = match json::from_str(response_text) {
        Ok(releases) => releases,
        Err(err) => {
            println!("{}", "Error:".red());
            println!("\n{response_text}");
            return Err(UserError::CannotDownload { url, reason: err.to_string() });
        }
    };
    let tags = tags.into_iter().map(Tag::standardized_tag).collect();
    Ok(tags)
}

/// data structure received from the GitHub API
#[derive(Deserialize, Debug, PartialEq)]
struct Tag {
    r#ref: String,
}

impl Tag {
    fn standardized_tag(self) -> String {
        match self.r#ref.strip_prefix("refs/tags/") {
            Some(stripped) => stripped.to_string(),
            None => self.r#ref,
        }
    }
}
