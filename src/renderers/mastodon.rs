use anyhow::Result;

use crate::{
    cli::RenderTarget, config::Config, post::CanonicalPost, renderers::RenderedPost,
    templates::render_template,
};

const DEFAULT_MASTODON_TEMPLATE: &str = r#"{excerpt}

New essay: {title}
{url}
"#;

const DEFAULT_MASTODON_MAX_CHARS: usize = 500;

pub fn render(post: &CanonicalPost, config: &Config) -> Result<RenderedPost> {
    let renderer_config = config.mastodon.as_ref();

    let template = post
        .template_override_for("mastodon")
        .or_else(|| renderer_config.and_then(|config| Some(config.template.as_str())))
        .unwrap_or(DEFAULT_MASTODON_TEMPLATE);

    let max_chars = renderer_config
        .and_then(|config| Some(config.max_chars))
        .unwrap_or(DEFAULT_MASTODON_MAX_CHARS);

    let body = render_template(&template, post, config)?;

    Ok(RenderedPost::new(
        RenderTarget::Mastodon,
        body,
        Some(max_chars),
        post.draft,
    ))
}
