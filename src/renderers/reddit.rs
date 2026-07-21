use anyhow::Result;

use crate::{
    config::{Config, RedditPostKind, RedditRendererConfig},
    post::CanonicalPost,
    renderers::{RenderedArtifact, RenderedPost},
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
        .map(normalize_subreddit);

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

    let (body, artifact) = match renderer_config.kind {
        RedditPostKind::Link => render_link_submission(
            post,
            config,
            &renderer_config,
            &title,
            subreddit,
            &mut warnings,
        )?,
        RedditPostKind::SelfPost => render_self_submission(
            post,
            config,
            &renderer_config,
            &title,
            subreddit,
            &mut warnings,
        )?,
    };

    let mut rendered =
        RenderedPost::new(RenderTarget::Reddit, body, None, post.draft, Some(artifact));
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
) -> Result<(String, RenderedArtifact)> {
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

    if let Some(comment) = comment.as_ref() {
        output.push_str("\n\nSuggested first comment:\n");
        output.push_str(comment);
    }

    output.push_str("\n\nReminder: check the subreddit rules before posting.");

    let artifact = RenderedArtifact::Reddit {
        subreddit: subreddit.map(str::to_string),
        kind: RedditPostKind::Link,
        title: title.to_string(),
        url: Some(url.to_string()),
        body: None,
        comment,
    };
    Ok((output, artifact))
}

fn render_self_submission(
    post: &CanonicalPost,
    config: &Config,
    renderer_config: &RedditRendererConfig,
    title: &str,
    subreddit: Option<&str>,
    warnings: &mut Vec<String>,
) -> Result<(String, RenderedArtifact)> {
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

    let artifact = RenderedArtifact::Reddit {
        subreddit: subreddit.map(str::to_string),
        kind: RedditPostKind::SelfPost,
        title: title.to_string(),
        url: None,
        body: Some(body),
        comment: None,
    };
    Ok((output, artifact))
}

fn normalize_subreddit(value: &str) -> &str {
    value
        .trim()
        .trim_start_matches("/r/")
        .trim_start_matches("r/")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_subreddit_values() {
        for (value, expected) in [
            ("example", "example"),
            ("r/example", "example"),
            ("/r/example", "example"),
            (" /r/example", "example"),
        ] {
            assert_eq!(normalize_subreddit(value), expected)
        }
    }

    #[test]
    fn renders_normalized_subreddit_values() {
        for value in ["example", "r/example", "/r/example"] {
            let config = Config {
                reddit: Some(RedditRendererConfig {
                    subreddit: Some(value.to_string()),
                    ..RedditRendererConfig::default()
                }),
                ..Config::default()
            };

            let rendered = render(&test_post(), &config).unwrap();

            assert!(
                rendered
                    .body
                    .starts_with("Subreddit: r/example\nKind: link\n\n"),
                "unexpected render for {value:?}: {}",
                rendered.body
            );

            let Some(RenderedArtifact::Reddit { subreddit, .. }) = rendered.artifact else {
                panic!("expected Reddit artifact for {value:?}");
            };

            assert_eq!(subreddit.as_deref(), Some("example"));
        }
    }

    fn test_post() -> CanonicalPost {
        CanonicalPost {
            title: "Example".to_string(),
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
