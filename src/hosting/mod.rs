//! This module accesses code hosting platforms on the internet.

pub(crate) mod github_releases;
pub(crate) mod github_tags;

/// provides the version of this release without "v" in it
///
/// NOTE: normally this function would only consume and produce a &str.
/// The way this function is used in this app, it's better to consume and provides an entire String.
/// This saves an allocation if the string doesn't have a leading v.
fn strip_leading_v(name: &str) -> &str {
  if let Some(stripped) = name.strip_prefix('v') { stripped } else { name }
}

#[cfg(test)]
mod tests {

  mod strip_leading_v {
    use super::super::strip_leading_v;

    #[test]
    fn leading_v() {
      assert_eq!(strip_leading_v("v1.2.3"), "1.2.3");
    }

    #[test]
    fn no_leading_v() {
      assert_eq!(strip_leading_v("1.2.3"), "1.2.3");
    }
  }
}
