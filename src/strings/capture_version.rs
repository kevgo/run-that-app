use crate::error::Result;
use crate::strings;

pub fn capture_version(text: &str) -> Result<&str> {
  strings::first_capture(text, r"(\d+\.\d+\.\d+)")
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
    let give = "1.1.1 or 2.2.2 or 3.3.3";
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
