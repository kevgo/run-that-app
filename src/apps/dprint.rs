use super::App;
use crate::hosting::GitHub;
use big_s::S;

pub struct Dprint {}

impl App for Dprint {
    fn executable(&self) -> &'static str {
        "dprint"
    }

    fn hoster(&self) -> Box<dyn crate::hosting::Hoster> {
        Box::new(GitHub {
            organization: String::from("dprint"),
            repo: String::from("dprint"),
        })
    }

    fn files_to_extract_from_archive(&self, _version: &str) -> Vec<String> {
        vec![S("dprint")]
    }
}
