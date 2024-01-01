use crate::{Output, Result, UserError};
use big_s::S;
use colored::Colorize;

pub fn all(org: &str, repo: &str, amount: usize, output: &dyn Output) -> Result<Vec<String>> {
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
    let Ok(response_text) = response.as_str() else {
        return Err(UserError::GitHubTagsApiProblem {
            problem: S("Cannot get response payload"),
            payload: S(""),
        });
    };
    let tags = parse_response(response_text)?;
    if tags.is_empty() {
        return Err(UserError::GitHubTagsApiProblem {
            problem: S("no tags found"),
            payload: S(""),
        });
    }
    Ok(tags)
}

fn parse_response(text: &str) -> Result<Vec<String>> {
    let value: serde_json::Value = serde_json::from_str(text).map_err(|err| UserError::GitHubTagsApiProblem {
        problem: err.to_string(),
        payload: text.to_string(),
    })?;
    let serde_json::Value::Array(entries) = value else {
        return Err(UserError::GitHubTagsApiProblem {
            problem: S("response doesn't contain an Array"),
            payload: text.to_string(),
        });
    };
    let mut result: Vec<String> = Vec::with_capacity(entries.len());
    for entry in entries {
        let Some(entry_ref) = entry["ref"].as_str() else {
            return Err(UserError::GitHubTagsApiProblem {
                problem: S("entry does not contain a ref field"),
                payload: entry.to_string(),
            });
        };
        if let Some(stripped) = entry_ref.strip_prefix("refs/tags/") {
            result.push(stripped.to_string());
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    mod parse_versions_response {
        use big_s::S;

        #[test]
        fn simple() {
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
            let have: Vec<String> = super::super::parse_response(response).unwrap();
            let want = vec![S("v1.0.1"), S("v1.0.2")];
            pretty::assert_eq!(have, want);
        }
    }
}
