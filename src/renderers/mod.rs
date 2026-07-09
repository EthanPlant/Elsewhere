use anyhow::Result;
use serde::Serialize;

use crate::{
    config::{Config, RedditPostKind},
    post::CanonicalPost,
    target::RenderTarget,
};

mod bluesky;
mod markdown;
mod mastodon;
mod reddit;

#[derive(Debug, Clone, Serialize)]
pub struct RenderedPost {
    pub target: RenderTarget,
    pub body: String,
    pub char_count: usize,
    pub max_chars: Option<usize>,
    pub warnings: Vec<String>,

    /// Additional structured metadata about the render
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<RenderedArtifact>,
}

impl RenderedPost {
    pub fn new(
        target: RenderTarget,
        body: String,
        max_chars: Option<usize>,
        draft: bool,
        artifact: Option<RenderedArtifact>,
    ) -> Self {
        let char_count = body.chars().count();
        let mut warnings = Vec::new();

        if let Some(max_chars) = max_chars
            && char_count > max_chars
        {
            warnings.push(format!(
                "Warning: {} render is {} characters. Configured limit is {}.",
                target, char_count, max_chars
            ));
        }

        if draft {
            warnings.push("Warning: post is marked as draft.".to_string());
        }

        Self {
            target,
            body,
            char_count,
            max_chars,
            warnings,
            artifact,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RenderedArtifact {
    Reddit {
        subreddit: Option<String>,
        kind: RedditPostKind,
        title: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },
}

pub fn render(target: RenderTarget, post: &CanonicalPost, config: &Config) -> Result<RenderedPost> {
    match target {
        RenderTarget::Mastodon => mastodon::render(post, config),
        RenderTarget::Bluesky => bluesky::render(post, config),
        RenderTarget::Markdown => markdown::render(post, config),
        RenderTarget::Reddit => reddit::render(post, config),
    }
}

fn choose_template<'a>(
    post: &'a CanonicalPost,
    target: RenderTarget,
    configured_template: Option<&'a str>,
    default_template: &'a str,
) -> &'a str {
    post.template_override_for(target.name())
        .or(configured_template)
        .unwrap_or(default_template)
}
