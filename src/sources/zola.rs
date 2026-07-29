use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::error::ElsewhereError;
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
        let path = self
            .find_zola_config()
            .ok_or(ElsewhereError::SourceConfigNotFound)?;

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read Zola config: {}", path.display()))?;

        let config: ZolaSiteConfig = toml::from_str(&raw)
            .with_context(|| format!("failed to parse Zola config: {}", path.display()))?;

        Ok(config)
    }

    fn find_zola_config(&self) -> Option<PathBuf> {
        let zola_toml = self.site_root.join("zola.toml");
        if zola_toml.is_file() {
            return Some(zola_toml);
        }
        let config_toml = self.site_root.join("config.toml");
        if config_toml.is_file() {
            return Some(config_toml);
        }

        None
    }
}

impl Source for ZolaSource {
    fn read_post(&self, path: &Path) -> Result<CanonicalPost> {
        markdown::read_markdown_post(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{assert_matches, fs};

    use tempfile::tempdir;

    #[test]
    fn uses_zola_toml_if_present() {
        let dir = tempdir().unwrap();

        let raw = "base_url = \"https://example.com\"";
        let file = dir.path().join("zola.toml");
        fs::write(file, raw).unwrap();

        let zola_source = ZolaSource::new(dir.path().to_path_buf());
        let zola_config = zola_source.read_config().unwrap();
        assert_eq!(zola_config.base_url, "https://example.com")
    }

    #[test]
    fn fallback_config_toml() {
        let dir = tempdir().unwrap();

        let raw = "base_url = \"https://example.com\"";
        let file = dir.path().join("config.toml");
        fs::write(file, raw).unwrap();

        let zola_source = ZolaSource::new(dir.path().to_path_buf());
        let zola_config = zola_source.read_config().unwrap();
        assert_eq!(zola_config.base_url, "https://example.com")
    }

    #[test]
    fn prefers_zola_toml() {
        let dir = tempdir().unwrap();

        let raw = "base_url = \"https://zola.toml\"";
        let file = dir.path().join("zola.toml");
        fs::write(file, raw).unwrap();

        let raw = "base_url = \"https://config.toml\"";
        let file = dir.path().join("config.toml");
        fs::write(file, raw).unwrap();

        let zola_source = ZolaSource::new(dir.path().to_path_buf());
        let zola_config = zola_source.read_config().unwrap();
        assert_eq!(zola_config.base_url, "https://zola.toml")
    }

    #[test]
    fn errors_if_no_config() {
        let dir = tempdir().unwrap();

        let zola_source = ZolaSource::new(dir.path().to_path_buf());
        let config_result = zola_source.read_config();
        assert_matches!(config_result, Err(_))
    }
}
