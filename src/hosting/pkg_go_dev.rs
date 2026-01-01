use crate::error::Result;
use crate::executables::Executable;
use crate::subshell;

pub(crate) fn all(pkg_name: &str, amount: usize) -> Result<Vec<String>> {
  // run "go list -m -versions golang.org/x/tools" in a subprocess
  let output = subshell::capture_output(&Executable::from("go"), &["list", "-m", "-versions", pkg_name])?;

  // parse the response
  parse_output(output)
}

fn parse_output(output: String) -> Result<Vec<String>> {
  Ok(vec![])
}

#[cfg(test)]
mod tests {
  use super::*;
  use big_s::S;

  #[test]
  fn deadcode() {
    let give = "golang.org/x/tools v0.1.0 v0.1.1 v0.1.2 v0.1.3 v0.1.4 v0.1.5 v0.1.6 v0.1.7 v0.1.8 v0.1.9 v0.1.10 v0.1.11 v0.1.12 v0.2.0 v0.3.0 v0.4.0 v0.5.0 v0.6.0 v0.7.0 v0.8.0 v0.9.0 v0.9.1 v0.9.2 v0.9.3 v0.10.0 v0.11.0 v0.11.1 v0.12.0 v0.13.0 v0.14.0 v0.15.0 v0.16.0 v0.16.1 v0.17.0 v0.18.0 v0.19.0 v0.20.0 v0.21.0 v0.22.0 v0.23.0 v0.24.0 v0.24.1 v0.25.0 v0.25.1 v0.26.0 v0.27.0 v0.28.0 v0.29.0 v0.30.0 v0.31.0 v0.32.0 v0.33.0 v0.34.0 v0.35.0 v0.36.0 v0.37.0 v0.38.0 v0.39.0 v0.40.0";
    let want = vec![
      S("v0.1.0"),
      S("v0.1.1"),
      S("v0.1.2"),
      S("v0.1.3"),
      S("v0.1.4"),
      S("v0.1.5"),
      S("v0.1.6"),
      S("v0.1.7"),
      S("v0.1.8"),
      S("v0.1.9"),
      S("v0.1.10"),
      S("v0.1.11"),
      S("v0.1.12"),
      S("v0.2.0"),
      S("v0.3.0"),
      S("v0.4.0"),
      S("v0.5.0"),
      S("v0.6.0"),
      S("v0.7.0"),
      S("v0.8.0"),
      S("v0.9.0"),
      S("v0.9.1"),
      S("v0.9.2"),
      S("v0.9.3"),
      S("v0.10.0"),
      S("v0.11.0"),
      S("v0.11.1"),
      S("v0.12.0"),
      S("v0.13.0"),
      S("v0.14.0"),
      S("v0.15.0"),
      S("v0.16.0"),
      S("v0.16.1"),
      S("v0.17.0"),
      S("v0.18.0"),
      S("v0.19.0"),
      S("v0.20.0"),
      S("v0.21.0"),
      S("v0.22.0"),
      S("v0.23.0"),
      S("v0.24.0"),
      S("v0.24.1"),
      S("v0.25.0"),
      S("v0.25.1"),
      S("v0.26.0"),
      S("v0.27.0"),
      S("v0.28.0"),
      S("v0.29.0"),
      S("v0.30.0"),
      S("v0.31.0"),
      S("v0.32.0"),
      S("v0.33.0"),
      S("v0.34.0"),
      S("v0.35.0"),
      S("v0.36.0"),
      S("v0.37.0"),
      S("v0.38.0"),
      S("v0.39.0"),
      S("v0.40.0"),
    ];
    let have = parse_output(give).unwrap();
    assert_eq!(have, want);
  }
}
