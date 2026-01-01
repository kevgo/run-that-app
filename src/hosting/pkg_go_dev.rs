use crate::configuration::Version;
use crate::error::{Result, UserError};
use crate::executables::Executable;
use crate::subshell;

pub(crate) fn latest(pkg_name: &str) -> Result<Version> {
  let tags = versions(pkg_name, 1)?;
  let Some(tag) = tags.into_iter().next() else {
    return Err(UserError::NoVersionsFound { app: pkg_name.to_string() });
  };
  Ok(tag)
}

pub(crate) fn versions(pkg_name: &str, amount: usize) -> Result<Vec<Version>> {
  let output = subshell::capture_output(&Executable::from("go"), &["list", "-m", "-versions", pkg_name])?;
  let mut versions = parse_output(&output);
  if versions.len() > amount {
    versions.resize(amount, Version::from(""));
  }
  Ok(versions)
}

fn parse_output(output: &str) -> Vec<Version> {
  let mut versions: Vec<Version> = output
    .split_whitespace()
    .skip(1) // skip the package name
    .map(|s| s.strip_prefix('v').unwrap_or(s).into())
    .collect();
  versions.sort_unstable_by(|a, b| b.cmp(a));
  versions
}

#[cfg(test)]
mod tests {

  mod parse_output {
    use super::super::*;
    use big_s::S;

    #[test]
    fn deadcode_all() {
      let give = "golang.org/x/tools v0.1.0 v0.1.1 v0.1.2 v0.1.3 v0.1.4 v0.1.5 v0.1.6 v0.1.7 v0.1.8 v0.1.9 v0.1.10 v0.1.11 v0.1.12 v0.2.0 v0.3.0 v0.4.0 v0.5.0 v0.6.0 v0.7.0 v0.8.0 v0.9.0 v0.9.1 v0.9.2 v0.9.3 v0.10.0 v0.11.0 v0.11.1 v0.12.0 v0.13.0 v0.14.0 v0.15.0 v0.16.0 v0.16.1 v0.17.0 v0.18.0 v0.19.0 v0.20.0 v0.21.0 v0.22.0 v0.23.0 v0.24.0 v0.24.1 v0.25.0 v0.25.1 v0.26.0 v0.27.0 v0.28.0 v0.29.0 v0.30.0 v0.31.0 v0.32.0 v0.33.0 v0.34.0 v0.35.0 v0.36.0 v0.37.0 v0.38.0 v0.39.0 v0.40.0";
      let want = vec![
        S("0.40.0"),
        S("0.39.0"),
        S("0.38.0"),
        S("0.37.0"),
        S("0.36.0"),
        S("0.35.0"),
        S("0.34.0"),
        S("0.33.0"),
        S("0.32.0"),
        S("0.31.0"),
        S("0.30.0"),
        S("0.29.0"),
        S("0.28.0"),
        S("0.27.0"),
        S("0.26.0"),
        S("0.25.1"),
        S("0.25.0"),
        S("0.24.1"),
        S("0.24.0"),
        S("0.23.0"),
        S("0.22.0"),
        S("0.21.0"),
        S("0.20.0"),
        S("0.19.0"),
        S("0.18.0"),
        S("0.17.0"),
        S("0.16.1"),
        S("0.16.0"),
        S("0.15.0"),
        S("0.14.0"),
        S("0.13.0"),
        S("0.12.0"),
        S("0.11.1"),
        S("0.11.0"),
        S("0.10.0"),
        S("0.9.3"),
        S("0.9.2"),
        S("0.9.1"),
        S("0.9.0"),
        S("0.8.0"),
        S("0.7.0"),
        S("0.6.0"),
        S("0.5.0"),
        S("0.4.0"),
        S("0.3.0"),
        S("0.2.0"),
        S("0.1.12"),
        S("0.1.11"),
        S("0.1.10"),
        S("0.1.9"),
        S("0.1.8"),
        S("0.1.7"),
        S("0.1.6"),
        S("0.1.5"),
        S("0.1.4"),
        S("0.1.3"),
        S("0.1.2"),
        S("0.1.1"),
        S("0.1.0"),
      ];
      let have = parse_output(give);
      assert_eq!(have, want);
    }
  }
}
