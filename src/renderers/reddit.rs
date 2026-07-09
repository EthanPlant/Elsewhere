use anyhow::Result;

use crate::{
    config::{Config, RedditPostKind, RedditRendererConfig},
    post::CanonicalPost,
    renderers::RenderedPost,
    target::RenderTarget,
    templates::render_template,
};

pub fn render(post: &CanonicalPost, config: &Config) -> Result<RenderedPost> {
    let renderer_config = effective_reddit_config(post, config);

    let title = render_template(&renderer_config.title_template, post, config)?;
    let title_count = title.chars().count();

    let subreddit = renderer_config
        .subreddit
        .as_deref()
        .map(|value| value.trim_start_matches("r/").to_string());

    let mut warnings = Vec::new();

    if title_count > renderer_config.title_max_chars {
        warnings.push(format!(
            "Warning: reddit title is {title_count} characters. Configured limit is {}.",
            renderer_config.title_max_chars
        ));
    }

    if subreddit.is_none() {
        warnings.push(
            "Warning: reddit subreddit is not configured. Check the destination community before posting."
                .to_string(),
        );
    }

    let body = match renderer_config.kind {
        RedditPostKind::Link => render_link_submission(
            post,
            config,
            &renderer_config,
            &title,
            subreddit.as_deref(),
            &mut warnings,
        )?,
        RedditPostKind::SelfPost => render_self_submission(
            post,
            config,
            &renderer_config,
            &title,
            subreddit.as_deref(),
            &mut warnings,
        )?,
    };

    let mut rendered = RenderedPost::new(RenderTarget::Reddit, body, None, post.draft);
    rendered.warnings.extend(warnings);
    Ok(rendered)
}

fn render_link_submission(
    post: &CanonicalPost,
    config: &Config,
    renderer_config: &RedditRendererConfig,
    title: &str,
    subreddit: Option<&str>,
    warnings: &mut Vec<String>,
) -> Result<String> {
    let url = post.canonical_url.as_deref().unwrap_or("");

    let comment = match &renderer_config.comment_template {
        Some(template) => {
            let comment = render_template(template, post, config)?;
            warn_if_over(
                warnings,
                "reddit comment",
                comment.chars().count(),
                renderer_config.comment_max_chars,
            );
            Some(comment)
        }
        None => None,
    };

    let mut output = String::new();

    push_header(&mut output, subreddit, "link");

    output.push_str("Title:\n");
    output.push_str(title);
    output.push_str("\n\nURL:\n");
    output.push_str(url);

    if let Some(comment) = comment {
        output.push_str("\n\nSuggested first comment:\n");
        output.push_str(&comment);
    }

    output.push_str("\n\nReminder: check the subreddit rules before posting.");
    Ok(output)
}

fn render_self_submission(
    post: &CanonicalPost,
    config: &Config,
    renderer_config: &RedditRendererConfig,
    title: &str,
    subreddit: Option<&str>,
    warnings: &mut Vec<String>,
) -> Result<String> {
    let template = renderer_config
        .body_template
        .as_deref()
        .unwrap_or("{excerpt}\n\n{url}");

    let body = render_template(template, post, config)?;
    warn_if_over(
        warnings,
        "reddit body",
        body.chars().count(),
        renderer_config.body_max_chars,
    );

    let mut output = String::new();

    push_header(&mut output, subreddit, "self");

    output.push_str("Title:\n");
    output.push_str(title);
    output.push_str("\n\nBody:\n");
    output.push_str(&body);

    output.push_str("\n\nReminder: check the subreddit rules before posting.");
    Ok(output)
}

fn push_header(output: &mut String, subreddit: Option<&str>, kind: &str) {
    output.push_str("Subreddit: ");
    match subreddit {
        Some(subreddit) => {
            output.push_str("r/");
            output.push_str(subreddit);
        }
        None => output.push_str("not configured"),
    }

    output.push_str("\nKind: ");
    output.push_str(kind);
    output.push_str("\n\n");
}

fn warn_if_over(warnings: &mut Vec<String>, label: &str, actual: usize, max: usize) {
    if actual > max {
        warnings.push(format!(
            "Warning: {label} is {actual} characters. Configured limit is {max}."
        ));
    }
}

fn effective_reddit_config(post: &CanonicalPost, config: &Config) -> RedditRendererConfig {
    let mut effective = config.reddit.clone().unwrap_or_default();

    if let Some(overrides) = post
        .elsewhere
        .as_ref()
        .and_then(|elsewhere| elsewhere.reddit.as_ref())
    {
        if let Some(subreddit) = overrides.subreddit.as_ref() {
            effective.subreddit = Some(subreddit.clone());
        }

        if let Some(kind) = overrides.kind {
            effective.kind = kind;
        }

        if let Some(title_template) = overrides.title_template.as_ref() {
            effective.title_template = title_template.clone();
        }

        if let Some(body_template) = overrides.body_template.as_ref() {
            effective.body_template = Some(body_template.clone());
        }

        if let Some(comment_template) = overrides.comment_template.as_ref() {
            effective.comment_template = Some(comment_template.clone());
        }
    }

    effective
}
