use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::error::ElsewhereError;
use crate::post::CanonicalPost;

pub const CONFIG_FILE_NAME: &str = "elsewhere.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    pub content_dir: String,
    pub source: SourceKind,
    pub defaults: Defaults,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic: Option<GenericConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zola: Option<ZolaConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastodon: Option<SocialRendererConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluesky: Option<SocialRendererConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<LongFormRendererConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reddit: Option<RedditRendererConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceKind {
    Generic,
    Zola,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Defaults {
    pub canonical_phrase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GenericConfig {
    pub url_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ZolaConfig {
    pub section_url_from_path: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SocialRendererConfig {
    pub max_chars: usize,
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LongFormRendererConfig {
    pub template: String,
}

impl Default for LongFormRendererConfig {
    fn default() -> Self {
        Self {
            template: "# {title}\n\n_{description}_\n\n{body}\n\n{canonical_phrase}\n{url}"
                .to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadedConfig {
    pub config: Config,
    pub root_dir: PathBuf,
}

impl Config {
    pub fn starter() -> Self {
        Self {
            site_url: Some("https://example.com".to_string()),
            content_dir: "content".to_string(),
            source: SourceKind::Generic,
            defaults: Defaults {
                canonical_phrase: "Originally published on my website:".to_string(),
            },

            generic: Some(GenericConfig::default()),
            zola: None,

            mastodon: None,
            bluesky: None,
            markdown: None,
            reddit: None,
        }
    }

    fn generic_config(&self) -> GenericConfig {
        self.generic.clone().unwrap_or_default()
    }

    fn zola_config(&self) -> ZolaConfig {
        self.zola.clone().unwrap_or_default()
    }

    pub fn derive_canonical_url(
        &self,
        site_url: &str,
        site_root: &Path,
        post_path: &Path,
        post: &CanonicalPost,
    ) -> Option<String> {
        if post.canonical_url.is_some() {
            return post.canonical_url.clone();
        }

        match self.source {
            SourceKind::Generic => self.derive_generic_url(site_url, post),
            SourceKind::Zola => self.derive_zola_url(site_url, site_root, post_path, post),
        }
    }

    fn derive_generic_url(&self, site_url: &str, post: &CanonicalPost) -> Option<String> {
        let slug = post.slug.as_deref()?;
        let generic_config = self.generic_config();

        let path = generic_config.url_pattern.replace("{slug}", slug);

        Some(join_site_url(site_url, &path))
    }

    fn derive_zola_url(
        &self,
        site_url: &str,
        site_root: &Path,
        post_path: &Path,
        post: &CanonicalPost,
    ) -> Option<String> {
        let zola_config = self.zola_config();

        if let Some(path) = &post.path {
            return Some(join_site_url(site_url, path));
        }

        if !zola_config.section_url_from_path {
            return self.derive_generic_url(site_url, post);
        }

        let content_dir = site_root.join(&self.content_dir);
        let relative = post_path.strip_prefix(&content_dir).ok()?;

        let mut parts: Vec<String> = relative
            .components()
            .filter_map(|component| component.as_os_str().to_str())
            .map(ToOwned::to_owned)
            .collect();

        let last = parts.pop()?;
        let filename_slug = Path::new(&last)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(&last)
            .to_string();

        let slug = post.slug.clone().unwrap_or(filename_slug);

        parts.push(slug);

        let path = format!("/{}/", parts.join("/"));
        Some(join_site_url(site_url, &path))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            site_url: None,
            content_dir: "content".to_string(),
            source: SourceKind::Generic,
            defaults: Defaults::default(),
            generic: None,
            zola: None,
            mastodon: None,
            bluesky: None,
            markdown: None,
            reddit: None,
        }
    }
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            canonical_phrase: "Originally published on my website:".to_string(),
        }
    }
}

impl Default for GenericConfig {
    fn default() -> Self {
        Self {
            url_pattern: "/writing/{slug}/".to_string(),
        }
    }
}

impl Default for ZolaConfig {
    fn default() -> Self {
        Self {
            section_url_from_path: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditRendererConfig {
    pub kind: RedditPostKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subreddit: Option<String>,

    pub title_template: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_template: Option<String>,

    pub title_max_chars: usize,
    pub body_max_chars: usize,
    pub comment_max_chars: usize,
}

impl Default for RedditRendererConfig {
    fn default() -> Self {
        Self {
            kind: RedditPostKind::Link,
            subreddit: None,
            title_template: "{title}".to_string(),
            body_template: Some("{excerpt}\n\n{url}".to_string()),
            comment_template: None,
            title_max_chars: 300,
            body_max_chars: 40_000,
            comment_max_chars: 10_000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RedditPostKind {
    Link,
    SelfPost,
}

impl FromStr for RedditPostKind {
    type Err = ElsewhereError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "link" => Ok(RedditPostKind::Link),
            "selfpost" => Ok(RedditPostKind::SelfPost),
            _ => Err(ElsewhereError::InvalidFrontMatterField {
                field: "reddit.kind",
                expected: "link or selfpost",
            }),
        }
    }
}

pub fn load_config_for_post(post_path: &Path) -> Result<LoadedConfig> {
    let start_dir = if post_path.is_absolute() {
        post_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("post path has no parent directory"))?
            .to_path_buf()
    } else {
        env::current_dir()
            .context("failed to determine current directory")?
            .join(post_path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("post path has no parent directory"))?
            .to_path_buf()
    };

    let path = find_config_file(&start_dir).ok_or(ElsewhereError::ConfigNotFound)?;

    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read configuration file: {}", path.display()))?;

    let config: Config = toml::from_str(&raw)
        .with_context(|| format!("failed to parse configuration file: {}", path.display()))?;

    let root_dir = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("configuration file has no parent directory"))?
        .to_path_buf();

    Ok(LoadedConfig { config, root_dir })
}
fn find_config_file(start_dir: &Path) -> Option<PathBuf> {
    let mut current = Some(start_dir);

    while let Some(dir) = current {
        let candidate = dir.join(CONFIG_FILE_NAME);

        if candidate.is_file() {
            return Some(candidate);
        }

        current = dir.parent();
    }

    None
}

fn join_site_url(site_url: &str, path: &str) -> String {
    let site_url = site_url.trim_end_matches('/');
    let path = path.trim_start_matches('/');

    format!("{site_url}/{path}")
}
