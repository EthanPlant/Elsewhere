// src/target.rs

use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RenderTarget {
    Mastodon,
    Bluesky,
    Markdown,
}

impl RenderTarget {
    pub fn all() -> &'static [RenderTarget] {
        &[Self::Mastodon, Self::Bluesky, Self::Markdown]
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Mastodon => "mastodon",
            Self::Bluesky => "bluesky",
            Self::Markdown => "markdown",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Mastodon => "Mastodon",
            Self::Bluesky => "Bluesky",
            Self::Markdown => "Markdown",
        }
    }

    pub fn is_long_form(self) -> bool {
        matches!(self, Self::Markdown)
    }
}

impl fmt::Display for RenderTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
