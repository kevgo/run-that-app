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
    let release: Release = match json::from_str(response_text) {
        Ok(release) => release,
        Err(err) => {
            println!("{}", "Error:".red());
            println!("\n{response_text}");
            return Err(UserError::CannotDownload { url, reason: err.to_string() });
        }
    };
    Ok(release.version().to_string())
}

pub fn versions(org: &str, repo: &str, amount: u8, output: &dyn Output) -> Result<Vec<String>> {}
