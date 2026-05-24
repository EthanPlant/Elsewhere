use std::path::Path;

use anyhow::Result;

use crate::post::CanonicalPost;

pub mod generic_markdown;

pub trait Source {
    fn read_post(&self, path: &Path) -> Result<CanonicalPost>;
}
