use super::strip_leading_v;
use crate::Output;
use crate::Result;
use crate::UserError;
use big_s::S;
use colored::Colorize;

/// provides the latest official version of the given application on GitHub Releases
pub fn latest(org: &str, repo: &str, output: &dyn Output) -> Result<String> {
    let url = format!("https://api.github.com/repos/{org}/{repo}/releases/latest");
    output.log("HTTP", &format!("downloading {url}"));
    let get = minreq::get(&url)
        .with_header("Accept", "application/vnd.github+json")
        .with_header("User-Agent", format!("run-that-app-{}", env!("CARGO_PKG_VERSION")))
        .with_header("X-GitHub-Api-Version", "2022-11-28");
    let Ok(response) = get.send() else {
        output.println(&format!("{}", "not online".red()));
        return Err(UserError::NotOnline);
    };
    let Ok(response_text) = response.as_str() else {
        return Err(UserError::GitHubReleasesApiProblem {
            problem: S("API response contains no body"),
            payload: S(""),
        });
    };
    parse_latest_response(response_text)
}

fn parse_latest_response(text: &str) -> Result<String> {
    let release: serde_json::Value = serde_json::from_str(text).map_err(|err| UserError::GitHubReleasesApiProblem {
        problem: err.to_string(),
        payload: text.to_string(),
    })?;
    Ok(strip_leading_v(release["tag_name"].as_str().unwrap()).to_string())
}

/// provides the given number of latest versions of the given application on GitHub Releases
pub fn versions(org: &str, repo: &str, amount: usize, output: &dyn Output) -> Result<Vec<String>> {
    let url = format!("https://api.github.com/repos/{org}/{repo}/releases?per_page={amount}");
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
    parse_versions_response(response.as_str().unwrap())
}

fn parse_versions_response(text: &str) -> Result<Vec<String>> {
    let releases: serde_json::Value = serde_json::from_str(text).map_err(|err| UserError::GitHubReleasesApiProblem {
        problem: err.to_string(),
        payload: text.to_string(),
    })?;
    let serde_json::Value::Array(releases) = releases else {
        return Err(UserError::GitHubReleasesApiProblem {
            problem: S("unknown API response: does not contain a list of releases"),
            payload: text.to_string(),
        });
    };
    let mut result: Vec<String> = Vec::with_capacity(releases.len());
    for release in releases {
        if let Some(release_tag) = release["tag_name"].as_str() {
            result.push(strip_leading_v(release_tag).to_string());
        };
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use big_s::S;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn parse_versions_response() {
        let response = r#"
[
  {
    "url": "https://api.github.com/repos/rhysd/actionlint/releases/121542388",
    "assets_url": "https://api.github.com/repos/rhysd/actionlint/releases/121542388/assets",
    "upload_url": "https://uploads.github.com/repos/rhysd/actionlint/releases/121542388/assets{?name,label}",
    "html_url": "https://github.com/rhysd/actionlint/releases/tag/v1.6.26",
    "id": 121542388,
    "author": {
      "login": "github-actions[bot]",
      "id": 41898282,
      "node_id": "MDM6Qm90NDE4OTgyODI=",
      "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
      "gravatar_id": "",
      "url": "https://api.github.com/users/github-actions%5Bbot%5D",
      "html_url": "https://github.com/apps/github-actions",
      "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
      "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
      "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
      "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
      "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
      "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
      "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
      "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
      "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
      "type": "Bot",
      "site_admin": false
    },
    "node_id": "RE_kwDOFhfz284HPpb0",
    "tag_name": "v1.6.26",
    "target_commitish": "main",
    "name": "v1.6.26",
    "draft": false,
    "prerelease": false,
    "created_at": "2023-09-18T14:03:08Z",
    "published_at": "2023-09-18T14:05:44Z",
    "assets": [
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581358",
        "id": 126581358,
        "node_id": "RA_kwDOFhfz284Hi3pu",
        "name": "actionlint_1.6.26_checksums.txt",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "text/plain; charset=utf-8",
        "state": "uploaded",
        "size": 1130,
        "download_count": 3956,
        "created_at": "2023-09-18T14:05:49Z",
        "updated_at": "2023-09-18T14:05:49Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_checksums.txt"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581347",
        "id": 126581347,
        "node_id": "RA_kwDOFhfz284Hi3pj",
        "name": "actionlint_1.6.26_darwin_amd64.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/gzip",
        "state": "uploaded",
        "size": 2018305,
        "download_count": 2542,
        "created_at": "2023-09-18T14:05:47Z",
        "updated_at": "2023-09-18T14:05:48Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_darwin_amd64.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581344",
        "id": 126581344,
        "node_id": "RA_kwDOFhfz284Hi3pg",
        "name": "actionlint_1.6.26_darwin_arm64.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/gzip",
        "state": "uploaded",
        "size": 1914495,
        "download_count": 3758,
        "created_at": "2023-09-18T14:05:47Z",
        "updated_at": "2023-09-18T14:05:48Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_darwin_arm64.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581341",
        "id": 126581341,
        "node_id": "RA_kwDOFhfz284Hi3pd",
        "name": "actionlint_1.6.26_freebsd_386.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/gzip",
        "state": "uploaded",
        "size": 1819290,
        "download_count": 2,
        "created_at": "2023-09-18T14:05:46Z",
        "updated_at": "2023-09-18T14:05:47Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_freebsd_386.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581342",
        "id": 126581342,
        "node_id": "RA_kwDOFhfz284Hi3pe",
        "name": "actionlint_1.6.26_freebsd_amd64.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/gzip",
        "state": "uploaded",
        "size": 1947301,
        "download_count": 1,
        "created_at": "2023-09-18T14:05:47Z",
        "updated_at": "2023-09-18T14:05:47Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_freebsd_amd64.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581329",
        "id": 126581329,
        "node_id": "RA_kwDOFhfz284Hi3pR",
        "name": "actionlint_1.6.26_linux_386.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/gzip",
        "state": "uploaded",
        "size": 1837375,
        "download_count": 17,
        "created_at": "2023-09-18T14:05:45Z",
        "updated_at": "2023-09-18T14:05:45Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_linux_386.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581338",
        "id": 126581338,
        "node_id": "RA_kwDOFhfz284Hi3pa",
        "name": "actionlint_1.6.26_linux_amd64.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/gzip",
        "state": "uploaded",
        "size": 1962980,
        "download_count": 185283,
        "created_at": "2023-09-18T14:05:46Z",
        "updated_at": "2023-09-18T14:05:46Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_linux_amd64.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581336",
        "id": 126581336,
        "node_id": "RA_kwDOFhfz284Hi3pY",
        "name": "actionlint_1.6.26_linux_arm64.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/gzip",
        "state": "uploaded",
        "size": 1789260,
        "download_count": 4488,
        "created_at": "2023-09-18T14:05:46Z",
        "updated_at": "2023-09-18T14:05:46Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_linux_arm64.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581330",
        "id": 126581330,
        "node_id": "RA_kwDOFhfz284Hi3pS",
        "name": "actionlint_1.6.26_linux_armv6.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/gzip",
        "state": "uploaded",
        "size": 1860560,
        "download_count": 4,
        "created_at": "2023-09-18T14:05:45Z",
        "updated_at": "2023-09-18T14:05:45Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_linux_armv6.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581352",
        "id": 126581352,
        "node_id": "RA_kwDOFhfz284Hi3po",
        "name": "actionlint_1.6.26_windows_386.zip",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/zip",
        "state": "uploaded",
        "size": 1995650,
        "download_count": 1019,
        "created_at": "2023-09-18T14:05:48Z",
        "updated_at": "2023-09-18T14:05:48Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_windows_386.zip"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581356",
        "id": 126581356,
        "node_id": "RA_kwDOFhfz284Hi3ps",
        "name": "actionlint_1.6.26_windows_amd64.zip",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/zip",
        "state": "uploaded",
        "size": 2084083,
        "download_count": 1266,
        "created_at": "2023-09-18T14:05:49Z",
        "updated_at": "2023-09-18T14:05:49Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_windows_amd64.zip"
      },
      {
        "url": "https://api.github.com/repos/rhysd/actionlint/releases/assets/126581351",
        "id": 126581351,
        "node_id": "RA_kwDOFhfz284Hi3pn",
        "name": "actionlint_1.6.26_windows_arm64.zip",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/zip",
        "state": "uploaded",
        "size": 1879198,
        "download_count": 425,
        "created_at": "2023-09-18T14:05:48Z",
        "updated_at": "2023-09-18T14:05:48Z",
        "browser_download_url": "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_windows_arm64.zip"
      }
    ],
    "tarball_url": "https://api.github.com/repos/rhysd/actionlint/tarball/v1.6.26",
    "zipball_url": "https://api.github.com/repos/rhysd/actionlint/zipball/v1.6.26",
    "body": "- Several template fields and template actions were added. All fields and actions are listed in [the document](https://github.com/rhysd/actionlint/blob/main/docs/usage.md#format-error-messages). Please read it for more details. (#311)\r\n  - By these additions, now actionlint can output the result in [the SARIF format](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html). SARIF is a format for the output of static analysis tools used by [GitHub CodeQL](https://codeql.github.com/). [the example Go template](https://github.com/rhysd/actionlint/blob/main/testdata/format/sarif_template.txt) to format actionlint output in SARIF.\r\n    ```sh\r\n    actionlint -format \"$(cat /path/to/sarif_template.txt)\" > output.json\r\n    ```\r\n  - `allKinds` returns the kinds (lint rules) information as an array. You can include what lint rules are defined in the command output.\r\n  - `toPascalCase` converts snake case (`foo_bar`) or kebab case (`foo-bar`) into pascal case (`FooBar`).\r\n- Report an error when the condition at `if:` is always evaluated to true. See [the check document](https://github.com/rhysd/actionlint/blob/main/docs/checks.md#if-cond-always-true) to know more details. (#272)\r\n  ```yaml\r\n  # ERROR: All the following `if:` conditions are always evaluated to true\r\n  - run: echo 'Commit is pushed'\r\n    if: |\r\n      ${{ github.event_name == 'push' }}\r\n  - run: echo 'Commit is pushed'\r\n    if: \"${{ github.event_name == 'push' }} \"\r\n  - run: echo 'Commit is pushed to main'\r\n    if: ${{ github.event_name == 'push' }} && ${{ github.ref_name == 'main' }}\r\n  ```\r\n- Fix actionlint didn't understand `${{ }}` placeholders in environment variable names. (#312)\r\n  ```yaml\r\n  env:\r\n    \"${{ steps.x.outputs.value }}\": \"...\"\r\n  ```\r\n- Fix type of matrix row when some expression is assigned to it with `${{ }}` (#285)\r\n  ```yaml\r\n  strategy:\r\n    matrix:\r\n      test:\r\n        # Matrix rows are assigned from JSON string\r\n        - ${{ fromJson(inputs.matrix) }}\r\n  steps:\r\n    - run: echo ${{ matrix.test.foo.bar }}\r\n  ```\r\n- Fix checking `exclude` of matrix was incorrect when some matrix row is dynamically constructed with `${{ }}`. (#261)\r\n  ```yaml\r\n  strategy:\r\n    matrix:\r\n      build-type:\r\n        - debug\r\n        - ${{ fromJson(inputs.custom-build-type) }}\r\n      exclude:\r\n        # 'release' is not listed in 'build-type' row, but it should not be reported as error\r\n        # since the second row of 'build-type' is dynamically constructed with ${{ }}.\r\n        - build-type: release\r\n  ```\r\n- Fix checking `exclude` of matrix was incorrect when object is nested at row of the matrix. (#249)\r\n  ```yaml\r\n  matrix:\r\n    os:\r\n      - name: Ubuntu\r\n        matrix: ubuntu\r\n      - name: Windows\r\n        matrix: windows\r\n    arch:\r\n      - name: ARM\r\n        matrix: arm\r\n      - name: Intel\r\n        matrix: intel\r\n    exclude:\r\n      # This should exclude { os: { name: Windows, matrix: windows }, arch: {name: ARM, matrix: arm } }\r\n      - os:\r\n          matrix: windows\r\n        arch:\r\n          matrix: arm\r\n  ```\r\n- Fix data race when `actionlint.yml` config file is used by multiple goroutines to check multiple workflow files. (#333)\r\n- Check keys' case sensitivity. (#302)\r\n  ```yaml\r\n  steps:\r\n    # ERROR: 'run:' is correct\r\n    - ruN: echo \"hello\"\r\n  ```\r\n- Add `number` as [input type of `workflow_dispatch` event](https://docs.github.com/en/actions/learn-github-actions/contexts#inputs-context). (#316)\r\n- Check max number of inputs of `workflow_dispatch` event is 10.\r\n- Check numbers at `timeout-minutes` and `max-parallel` are greater than zero.\r\n- Add Go APIs to define a custom rule. Please read [the code example](https://pkg.go.dev/github.com/rhysd/actionlint/#example_Linter_yourOwnRule) to know the usage.\r\n  - Make some [`RuleBase`](https://pkg.go.dev/github.com/rhysd/actionlint#RuleBase) methods public which are useful to implement your own custom rule type. (thanks @hugo-syn, #327, #331)\r\n  - `OnRulesCreated` field is added to [`LinterOptions`](https://pkg.go.dev/github.com/rhysd/actionlint#LinterOptions) struct. You can modify applied rules with the hook (add your own rule, remove some rule, ...).\r\n- Add `NewProject()` Go API to create a [`Project`](https://pkg.go.dev/github.com/rhysd/actionlint#Project) instance.\r\n- Fix tests failed when sources are downloaded from `.tar.gz` link. (#307)\r\n- Improve [the pre-commit document](https://github.com/rhysd/actionlint/blob/main/docs/usage.md#pre-commit) to explain all pre-commit hooks by this repository.\r\n- Clarify the regular expression syntax of `-ignore` option is [RE2](https://github.com/google/re2/wiki/Syntax). (#320)\r\n- Use ubuntu-latest runner to create winget release. (thanks @sitiom, #308)\r\n- Update popular actions data set, available contexts, webhook types to the latest.\r\n  - Fix typo in `watch` webhook's types (thanks @suzuki-shunsuke, #334)\r\n  - Add `secret_source` property to [`github` context](https://docs.github.com/en/actions/learn-github-actions/contexts#github-context). (thanks @asml-mdroogle, #339)\r\n  - Many new major releases are added to the popular actions data set (including `actions/checkout@v4`).\r\n- Use Go 1.21 to build release binaries.\r\n- Update Go dependencies to the latest. (thanks @harryzcy, #322)",
    "reactions": {
      "url": "https://api.github.com/repos/rhysd/actionlint/releases/121542388/reactions",
      "total_count": 9,
      "+1": 1,
      "-1": 0,
      "laugh": 0,
      "hooray": 2,
      "confused": 0,
      "heart": 6,
      "rocket": 0,
      "eyes": 0
    },
    "mentions_count": 5
  }
]"#;
        let have: Vec<String> = super::parse_versions_response(response).unwrap();
        let want = vec!["1.6.26"];
        assert_eq!(have, want);
    }

    #[test]
    fn parse_latest_response() {
        let response = r#"{
  "url": "https://api.github.com/repos/octocat/Hello-World/releases/1",
  "html_url": "https://github.com/octocat/Hello-World/releases/v1.0.0",
  "assets_url": "https://api.github.com/repos/octocat/Hello-World/releases/1/assets",
  "upload_url": "https://uploads.github.com/repos/octocat/Hello-World/releases/1/assets{?name,label}",
  "tarball_url": "https://api.github.com/repos/octocat/Hello-World/tarball/v1.0.0",
  "zipball_url": "https://api.github.com/repos/octocat/Hello-World/zipball/v1.0.0",
  "discussion_url": "https://github.com/octocat/Hello-World/discussions/90",
  "id": 1,
  "node_id": "MDc6UmVsZWFzZTE=",
  "tag_name": "v1.0.0",
  "target_commitish": "master",
  "name": "v1.0.0",
  "body": "Description of the release",
  "draft": false,
  "prerelease": false,
  "created_at": "2013-02-27T19:35:32Z",
  "published_at": "2013-02-27T19:35:32Z",
  "author": {
    "login": "octocat",
    "id": 1,
    "node_id": "MDQ6VXNlcjE=",
    "avatar_url": "https://github.com/images/error/octocat_happy.gif",
    "gravatar_id": "",
    "url": "https://api.github.com/users/octocat",
    "html_url": "https://github.com/octocat",
    "followers_url": "https://api.github.com/users/octocat/followers",
    "following_url": "https://api.github.com/users/octocat/following{/other_user}",
    "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
    "organizations_url": "https://api.github.com/users/octocat/orgs",
    "repos_url": "https://api.github.com/users/octocat/repos",
    "events_url": "https://api.github.com/users/octocat/events{/privacy}",
    "received_events_url": "https://api.github.com/users/octocat/received_events",
    "type": "User",
    "site_admin": false
  },
  "assets": [
    {
      "url": "https://api.github.com/repos/octocat/Hello-World/releases/assets/1",
      "browser_download_url": "https://github.com/octocat/Hello-World/releases/download/v1.0.0/example.zip",
      "id": 1,
      "node_id": "MDEyOlJlbGVhc2VBc3NldDE=",
      "name": "example.zip",
      "label": "short description",
      "state": "uploaded",
      "content_type": "application/zip",
      "size": 1024,
      "download_count": 42,
      "created_at": "2013-02-27T19:35:32Z",
      "updated_at": "2013-02-27T19:35:32Z",
      "uploader": {
        "login": "octocat",
        "id": 1,
        "node_id": "MDQ6VXNlcjE=",
        "avatar_url": "https://github.com/images/error/octocat_happy.gif",
        "gravatar_id": "",
        "url": "https://api.github.com/users/octocat",
        "html_url": "https://github.com/octocat",
        "followers_url": "https://api.github.com/users/octocat/followers",
        "following_url": "https://api.github.com/users/octocat/following{/other_user}",
        "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
        "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
        "organizations_url": "https://api.github.com/users/octocat/orgs",
        "repos_url": "https://api.github.com/users/octocat/repos",
        "events_url": "https://api.github.com/users/octocat/events{/privacy}",
        "received_events_url": "https://api.github.com/users/octocat/received_events",
        "type": "User",
        "site_admin": false
      }
    }
  ]
}"#;
        let have: String = super::parse_latest_response(response).unwrap();
        let want = S("1.0.0");
        assert_eq!(have, want);
    }
}
