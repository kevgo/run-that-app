use crate::error::Result;
use crate::strings;

/// provides the first match of three numbers separated by dots in the given text
pub fn first_version(text: &str) -> Result<&str> {
  strings::first_capture(text, r"(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use super::first_version;
  use crate::error::UserError;
  use maplit::hashmap;

  #[test]
  fn test() {
    let tests = hashmap! {
        "1.2.3" => Ok("1.2.3"),
        "foo 1.2.3 bar" => Ok("1.2.3"),
        "1.1.1 or 2.2.2 or 3.3.3" => Ok("1.1.1"),
        "word1 word2" => Err(UserError::RegexDoesntMatch),
    };
    for (give, want) in tests {
      let have = first_version(give);
      assert_eq!(have, want, "{give} -> {want:?}");
    }
  }
}
