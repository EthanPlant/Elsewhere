use anyhow::Result;

use crate::{
    config::Config,
    post::CanonicalPost,
    renderers::{RenderedPost, choose_template},
    target::RenderTarget,
    templates::render_template,
};

const DEFAULT_TEMPLATE: &str = r#"# {title}

_{description}_

{body}

{canonical_phrase}
{url}"#;

pub fn render(post: &CanonicalPost, config: &Config) -> Result<RenderedPost> {
    let renderer_config = config.markdown.as_ref();

    let template = choose_template(
        post,
        RenderTarget::Markdown,
        renderer_config.map(|config| config.template.as_str()),
        DEFAULT_TEMPLATE,
    );

    let body = render_template(template, post, config)?;

    Ok(RenderedPost::new(
        RenderTarget::Markdown,
        body,
        None,
        post.draft,
    ))
}
