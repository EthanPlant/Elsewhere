use anyhow::Result;

use crate::{
    cli::RenderTarget, config::Config, post::CanonicalPost, renderers::RenderedPost,
    templates::render_template,
};

pub fn render(post: &CanonicalPost, config: &Config) -> Result<RenderedPost> {
    let body = render_template(&config.bluesky.template, post, config)?;

    Ok(RenderedPost::new(
        RenderTarget::Bluesky,
        body,
        Some(config.mastodon.max_chars),
        post.draft,
    ))
}
