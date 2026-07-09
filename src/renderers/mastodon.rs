use anyhow::Result;

use crate::{
    config::Config,
    post::CanonicalPost,
    renderers::{RenderedPost, choose_template},
    target::RenderTarget,
    templates::render_template,
};

const DEFAULT_TEMPLATE: &str = r#"{excerpt}

New post: {title}
{url}
"#;

const DEFAULT_MAX_CHARS: usize = 500;

pub fn render(post: &CanonicalPost, config: &Config) -> Result<RenderedPost> {
    let renderer_config = config.mastodon.as_ref();

    let template = choose_template(
        post,
        RenderTarget::Mastodon,
        renderer_config.map(|config| config.template.as_str()),
        DEFAULT_TEMPLATE,
    );

    let max_chars = renderer_config
        .map(|config| config.max_chars)
        .unwrap_or(DEFAULT_MAX_CHARS);

    let body = render_template(template, post, config)?;

    Ok(RenderedPost::new(
        RenderTarget::Mastodon,
        body,
        Some(max_chars),
        post.draft,
        None,
    ))
}
