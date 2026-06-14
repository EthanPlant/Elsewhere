use anyhow::Result;

use crate::{
    cli::RenderTarget, config::Config, post::CanonicalPost, renderers::RenderedPost,
    templates::render_template,
};

pub fn render(post: &CanonicalPost, config: &Config) -> Result<RenderedPost> {
    let body = render_template(&config.substack.template, post, config)?;

    Ok(RenderedPost::new(
        RenderTarget::Substack,
        body,
        None,
        post.draft,
    ))
}
