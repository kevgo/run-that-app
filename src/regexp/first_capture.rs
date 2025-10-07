use crate::error::{Result, UserError};
use regex::Regex;

/// provides the first capture of the given regex in the given text
pub(crate) fn first_capture<'a>(text: &'a str, regex: &str) -> Result<&'a str> {
  let regex = Regex::new(regex).map_err(|err| UserError::InvalidRegex {
    regex: regex.to_string(),
    err: err.to_string(),
  })?;
  let Some(captures) = regex.captures(text) else {
    return Err(UserError::RegexDoesntMatch);
  };
  let Some(first_capture) = captures.get(1) else {
    return Err(UserError::RegexHasNoCaptures);
  };
  Ok(first_capture.as_str())
}

#[cfg(test)]
mod tests {
  use super::first_capture;
  use crate::UserError;

  #[test]
  fn multiple_matches() {
    let text = "You can run 1.1 or 1.2 or 1.3";
    let have = first_capture(text, r"(\d+\.\d+)");
    let want = Ok("1.1");
    assert_eq!(have, want);
  }

  #[test]
  fn no_match() {
    let text = "word1 word2";
    let have = first_capture(text, r"(\d+\.\d+)");
    let want = Err(UserError::RegexDoesntMatch);
    assert_eq!(have, want);
  }

  #[test]
  fn no_capture() {
    let text = "word1 word2";
    let have = first_capture(text, r"word1 word2");
    let want = Err(UserError::RegexHasNoCaptures);
    assert_eq!(have, want);
  }
}
