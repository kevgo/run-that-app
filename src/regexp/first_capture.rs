use crate::prelude::*;
use regex::Regex;

/// provides the first capture of the given regex in the given text
pub fn first_capture<'a>(text: &'a str, regex: &str) -> Result<&'a str> {
  let regex = Regex::new(regex).map_err(|err| UserError::InvalidRegex {
    regex: regex.to_string(),
    err: err.to_string(),
  })?;
  let Some(captures) = regex.captures(text) else {
    return Err(UserError::RegexHasNoCaptures { regex: regex.to_string() });
  };
  let Some(first_capture) = captures.get(1) else {
    return Err(UserError::RegexHasNoCaptures { regex: regex.to_string() });
  };
  Ok(first_capture.as_str())
}

#[cfg(test)]
mod tests {
  use super::first_capture;
  use crate::regexp::first_capture::UserError;
  use big_s::S;

  #[test]
  fn multiple_matches() {
    let text = "You can run 1.1 or 1.2 or 1.3";
    let have = first_capture(text, r"(\d+\.\d+)");
    let want = Ok("1.1");
    assert_eq!(have, want);
  }

  #[test]
  fn no_match() {
    let text = "Foo bar";
    let re = r"(\d+\.\d+";
    let have = first_capture(text, re);
    let want = Err(UserError::RegexHasNoCaptures { regex: S(re) });
    assert_eq!(have, want);
  }

  #[test]
  fn no_capture() {
    let text = "Foo bar";
    let have = first_capture(text, r"no capture");
    let want = Err(UserError::RegexHasNoCaptures { regex: S("no capture") });
    assert_eq!(have, want);
  }
}
