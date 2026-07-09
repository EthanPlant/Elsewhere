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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::SocialRendererConfig,
        post::{ElsewhereFrontMatter, ElsewhereTargetOverride},
    };

    #[test]
    fn mastodon_warns_when_over_character_limit() {
        let config = Config {
            mastodon: Some(SocialRendererConfig {
                max_chars: 10,
                template: "{title}".to_string(),
            }),
            ..Config::default()
        };

        let post = test_post("This title is longer than ten characters");

        let rendered = render(RenderTarget::Mastodon, &post, &config).unwrap();

        assert_eq!(rendered.max_chars, Some(10));
        assert!(!rendered.warnings.is_empty());
    }

    #[test]
    fn bluesky_warns_when_over_character_limit() {
        let config = Config {
            bluesky: Some(SocialRendererConfig {
                max_chars: 10,
                template: "{title}".to_string(),
            }),
            ..Config::default()
        };

        let post = test_post("This title is longer than ten characters");

        let rendered = render(RenderTarget::Bluesky, &post, &config).unwrap();

        assert_eq!(rendered.max_chars, Some(10));
        assert!(!rendered.warnings.is_empty());
    }

    #[test]
    fn platform_specific_override_wins() {
        let config = Config {
            mastodon: Some(SocialRendererConfig {
                max_chars: 500,
                template: "Site-level template: {title}".to_string(),
            }),
            ..Config::default()
        };

        let mut post = test_post("Example");
        post.elsewhere = Some(ElsewhereFrontMatter {
            excerpt: Some("Custom excerpt.".to_string()),
            mastodon: Some(ElsewhereTargetOverride {
                template: Some("Post-level template: {excerpt}".to_string()),
            }),
            bluesky: None,
            markdown: None,
            reddit: None,
        });

        let rendered = render(RenderTarget::Mastodon, &post, &config).unwrap();

        assert_eq!(rendered.body, "Post-level template: Custom excerpt.");
    }

    fn test_post(title: &str) -> CanonicalPost {
        CanonicalPost {
            title: title.to_string(),
            description: Some("Description.".to_string()),
            date: None,
            tags: Vec::new(),
            canonical_url: Some("https://example.com/writing/example/".to_string()),
            body_markdown: "Body.".to_string(),
            first_paragraph: Some("First paragraph.".to_string()),
            slug: Some("example".to_string()),
            elsewhere: None,
            path: None,
            draft: false,
        }
    }
}
