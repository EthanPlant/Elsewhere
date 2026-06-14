use std::path::Path;

use anyhow::Result;

use crate::post::CanonicalPost;

pub mod generic;
pub mod markdown;
pub mod zola;

pub trait Source {
    fn read_post(&self, path: &Path) -> Result<CanonicalPost>;
}
