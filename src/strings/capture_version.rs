use crate::error::{Result, UserError};
use regex::Regex;

pub fn capture_version(text: &str) -> Result<&str> {
  let regex = r"(\d+\.\d+\.\d+)";
  let regex = Regex::new(regex).map_err(|err| UserError::InvalidRegex {
    regex: regex.to_string(),
    err: err.to_string(),
  })?;
  let Some(captures) = regex.captures(text) else {
    return Err(UserError::RegexDoesntMatch);
  };
  let Some(first_capture) = captures.get(0) else {
    return Err(UserError::RegexHasNoCaptures);
  };
  Ok(first_capture.as_str())
}

#[cfg(test)]
mod tests {
  use super::capture_version;
  use crate::UserError;

  #[test]
  fn exact_match() {
    let give = "1.2.3";
    let have = capture_version(give);
    let want = Ok("1.2.3");
    assert_eq!(have, want);
  }

  #[test]
  fn exact_match_with_text() {
    let give = "foo 1.2.3 bar";
    let have = capture_version(give);
    let want = Ok("1.2.3");
    assert_eq!(have, want);
  }

  #[test]
  fn multiple_matches() {
    let give = "1.1.l or 2.2.2 or 3.3.3";
    let have = capture_version(give);
    let want = Ok("1.1.1");
    assert_eq!(have, want);
  }

  #[test]
  fn no_match() {
    let text = "word1 word2";
    let have = capture_version(text);
    let want = Err(UserError::RegexDoesntMatch);
    assert_eq!(have, want);
  }
}
