use super::{create, load, Yard};
use crate::Result;
use std::path::Path;

pub fn load_or_create(containing_folder: &Path) -> Result<Yard> {
  match load(containing_folder)? {
    Some(existing_yard) => Ok(existing_yard),
    None => create(containing_folder),
  }
}
