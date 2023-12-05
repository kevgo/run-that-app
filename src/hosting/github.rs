use crate::download::http_get;
use crate::Output;
use crate::Result;

pub fn versions(org: &str, repo: &str, output: &dyn Output) -> Result<Vec<String>> {
    let url = format!("https://api.github.com/repos/{org}/{repo}/releases");
    let Some(data) = http_get(&url, output)? else {
        panic!("GitHub API 404");
    };
    println!("{}", String::from_utf8_lossy(&data));
    Ok(vec![])
}
