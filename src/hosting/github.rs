use crate::Output;
use crate::Result;
use crate::UserError;
use colored::Colorize;

pub fn versions(org: &str, repo: &str, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
    let url = format!("https://api.github.com/repos/{org}/{repo}/releases?per_page={amount}");
    let get = minreq::get(url)
        .with_param("per_page", amount.to_string())
        .with_header("Accept", "application/vnd.github+json")
        .with_header("User-Agent", format!("run-that-app-{}", env!("CARGO_PKG_VERSION")))
        .with_header("X-GitHub-Api-Version", "2022-11-28");
    let Ok(response) = get.send() else {
        output.println(&format!("{}", "not online".red()));
        return Err(UserError::NotOnline);
    };
    println!("{}", response.as_str().unwrap());
    Ok(vec![])
}
