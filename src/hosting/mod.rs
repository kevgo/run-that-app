pub mod github_releases;
pub mod github_tags;

/// provides the version of this release without "v" in it
///
/// NOTE: normally this function would only consume and produce a &str.
/// The way this function is used in this app, it's better to consume and provides an entire String.
/// This saves an allocation if the string doesn't have a leading v.
fn strip_leading_v(name: String) -> String {
    if let Some(stripped) = name.strip_prefix('v') {
        stripped.to_string()
    } else {
        name
    }
}

#[cfg(test)]
mod tests {

    mod strip_leading_v {
        use super::super::strip_leading_v;
        use big_s::S;

        #[test]
        fn leading_v() {
            assert_eq!(strip_leading_v(S("v1.2.3")), "1.2.3");
        }

        #[test]
        fn no_leading_v() {
            assert_eq!(strip_leading_v(S("1.2.3")), "1.2.3");
        }
    }
}
