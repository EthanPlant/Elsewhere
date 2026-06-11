use anyhow::Result;

use crate::{cli::RenderTarget, config::Config, post::CanonicalPost};

mod bluesky;
mod mastodon;
mod substack;

#[derive(Debug, Clone)]
pub struct RenderedPost {
    pub target: RenderTarget,
    pub body: String,
    pub char_count: usize,
    pub max_chars: Option<usize>,
    pub warnings: Vec<String>,
}

impl RenderedPost {
    pub fn new(target: RenderTarget, body: String, max_chars: Option<usize>) -> Self {
        let char_count = body.chars().count();
        let mut warnings = Vec::new();

        if let Some(max_chars) = max_chars {
            if char_count > max_chars {
                warnings.push(format!(
                    "Warning: {} render is {} characters. Configured limit is {}.",
                    target, char_count, max_chars
                ));
            }
        }

        Self {
            target,
            body,
            char_count,
            max_chars,
            warnings,
        }
    }
}

pub fn render(
    target: &RenderTarget,
    post: &CanonicalPost,
    config: &Config,
) -> Result<RenderedPost> {
    match target {
        RenderTarget::Bluesky => bluesky::render(post, config),
        RenderTarget::Mastodon => mastodon::render(post, config),
        RenderTarget::Substack => substack::render(post, config),
    }
}
