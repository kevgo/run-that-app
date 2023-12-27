use super::strip_leading_v;
use crate::{Output, Result, UserError};
use big_s::S;
use colored::Colorize;

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
    parse_latest_response(response.as_str().unwrap(), url)
}

fn parse_latest_response(text: &str, url: String) -> Result<String> {
    let release: serde_json::Value = serde_json::from_str(text).map_err(|err| UserError::CannotParseApiResponse {
        reason: err.to_string(),
        text: text.to_string(),
        url,
    })?;
    Ok(strip_leading_v(release["ref"].as_str().unwrap()).to_string())
}

pub fn versions(org: &str, repo: &str, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
    let url = format!("https://api.github.com/repos/{org}/{repo}/git/refs/tags");
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
    parse_versions_response(response.as_str().unwrap(), url)
}

fn parse_versions_response(text: &str, url: String) -> Result<Vec<String>> {
    let value: serde_json::Value = serde_json::from_str(text).map_err(|err| UserError::CannotParseApiResponse {
        reason: err.to_string(),
        text: text.to_string(),
        url: url.clone(),
    })?;
    let serde_json::Value::Array(entries) = value else {
        return Err(UserError::CannotParseApiResponse {
            reason: S("response from GitHub API to load tags doesn't contain an Array"),
            text: text.to_string(),
            url,
        });
    };
    let mut result: Vec<String> = Vec::with_capacity(entries.len());
    for entry in entries {
        let Some(entry_ref) = entry["ref"].as_str() else {
            return Err(UserError::CannotParseApiResponse {
                reason: S("entry does not contain a ref field"),
                text: entry.to_string(),
                url,
            });
        };
        if !entry_ref.starts_with("refs/tags/") {
            continue;
        }
        let tag = entry_ref[10..].to_string();
        result.push(tag);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    mod parse_versions_response {
        use big_s::S;

        #[test]
        fn parse_versions_response() {
            let response = r#"
[
  {
    "ref": "refs/tags/v1.0.1",
    "node_id": "MDM6UmVmMjMwOTY5NTk6cmVmcy90YWdzL2dvMS4wLjE=",
    "url": "https://api.github.com/repos/foo/bar/git/refs/tags/v1.0.1",
    "object": {
      "sha": "2fffba7fe19690e038314d17a117d6b87979c89f",
      "type": "commit",
      "url": "https://api.github.com/repos/foo/bar/git/commits/2fffba7fe19690e038314d17a117d6b87979c89f"
    }
  },
  {
    "ref": "refs/tags/v1.0.2",
    "node_id": "MDM6UmVmMjMwOTY5NTk6cmVmcy90YWdzL2dvMS4wLjE=",
    "url": "https://api.github.com/repos/foo/bar/git/refs/tags/v1.0.2",
    "object": {
      "sha": "2fffba7fe19690e038314d17a117d6b87979c89f",
      "type": "commit",
      "url": "https://api.github.com/repos/foo/bar/git/commits/2fffba7fe19690e038314d17a117d6b87979c89f"
    }
  }
]

            "#;
            let have: Vec<String> = super::super::parse_versions_response(response, S("url")).unwrap();
            let want = vec![S("v1.0.1"), S("v1.0.2")];
            pretty::assert_eq!(have, want)
        }
    }
}
