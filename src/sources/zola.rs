use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::post::CanonicalPost;
use crate::sources::Source;
use crate::sources::markdown;

#[derive(Debug, Clone)]
pub struct ZolaSource {
    pub site_root: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZolaSiteConfig {
    pub base_url: String,
}

impl ZolaSource {
    pub fn new(site_root: PathBuf) -> Self {
        Self { site_root }
    }

    pub fn read_config(&self) -> Result<ZolaSiteConfig> {
        let path = self.site_root.join("zola.toml");

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read Zola config: {}", path.display()))?;

        let config: ZolaSiteConfig = toml::from_str(&raw)
            .with_context(|| format!("failed to parse Zola config: {}", path.display()))?;

        Ok(config)
    }
}

impl Source for ZolaSource {
    fn read_post(&self, path: &Path) -> Result<CanonicalPost> {
        markdown::read_markdown_post(path)
    }
}
