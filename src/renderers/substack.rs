use anyhow::Result;

use crate::{
    cli::RenderTarget, config::Config, post::CanonicalPost, renderers::RenderedPost,
    templates::render_template,
};

const DEFAULT_SUBSTACK_TEMPLATE: &str = r#"# {title}

_{description}_

{body}

{canonical_phrase}
{url}"#;

pub fn render(post: &CanonicalPost, config: &Config) -> Result<RenderedPost> {
    let renderer_config = config.substack.as_ref();

    let template = post
        .template_override_for("substack")
        .or_else(|| renderer_config.and_then(|config| Some(config.template.as_str())))
        .unwrap_or(DEFAULT_SUBSTACK_TEMPLATE);

    let body = render_template(&template, post, config)?;

    Ok(RenderedPost::new(
        RenderTarget::Substack,
        body,
        None,
        post.draft,
    ))
}
