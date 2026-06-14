use std::path::Path;

use anyhow::Result;

use crate::post::CanonicalPost;
use crate::sources::Source;
use crate::sources::markdown;

pub struct GenericSource;

impl Source for GenericSource {
    fn read_post(&self, path: &Path) -> Result<CanonicalPost> {
        markdown::read_markdown_post(path)
    }
}
