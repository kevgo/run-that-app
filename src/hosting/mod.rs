//! This module accesses code hosting platforms on the internet.

pub(crate) mod github_releases;
pub(crate) mod github_tags;
pub(crate) mod pkg_go_dev;

/// provides the version of this release without "v" in it
///
/// NOTE: normally this function would only consume and produce a &str.
/// The way this function is used in this app, it's better to consume and provides an entire String.
/// This saves an allocation if the string doesn't have a leading v.
fn strip_prefix<'a>(name: &'a str, prefix: &str) -> &'a str {
  name.strip_prefix(prefix).unwrap_or(name)
}

#[cfg(test)]
mod tests {

  mod strip_leading_v {
    use super::super::strip_prefix;

    #[test]
    fn leading_v() {
      assert_eq!(strip_prefix("v1.2.3", "v"), "1.2.3");
    }

    #[test]
    fn no_leading_v() {
      assert_eq!(strip_prefix("1.2.3", "v"), "1.2.3");
    }

    #[test]
    fn no_prefix() {
      assert_eq!(strip_prefix("1.2.3", ""), "1.2.3");
    }
  }
}
