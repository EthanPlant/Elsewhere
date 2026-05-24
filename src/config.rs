use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub site_url: String,
    pub content_dir: String,
    pub source: SourceKind,
    pub defaults: Defaults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceKind {
    GenericMarkdown,
    Zola,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaults {
    pub canonical_phrase: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            site_url: "https://example.com".to_string(),
            content_dir: "content".to_string(),
            source: SourceKind::GenericMarkdown,
            defaults: Defaults {
                canonical_phrase: "Originally published on my website".to_string(),
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
