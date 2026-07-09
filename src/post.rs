use crate::config::RedditPostKind;

#[derive(Debug, Clone)]
pub struct CanonicalPost {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub canonical_url: Option<String>,
    pub body_markdown: String,
    pub first_paragraph: Option<String>,
    pub slug: Option<String>,
    /// Elsewhere-specific frontmatter
    pub elsewhere: Option<ElsewhereFrontMatter>,

    // Zola/font-matter page metadata
    pub path: Option<String>,
    pub draft: bool,
}

impl CanonicalPost {
    pub fn first_paragraph_from_markdown(markdown: &str) -> Option<String> {
        markdown
            .split("\n\n")
            .map(str::trim)
            .find(|paragraph| !paragraph.is_empty())
            .map(ToOwned::to_owned)
    }

    pub fn editorial_excerpt(&self) -> String {
        self.elsewhere
            .as_ref()
            .and_then(|elsewhere| elsewhere.excerpt.as_deref())
            .or(self.description.as_deref())
            .or(self.first_paragraph.as_deref())
            .unwrap_or(self.title.as_str())
            .to_string()
    }

    pub fn template_override_for(&self, target: &str) -> Option<&str> {
        self.elsewhere
            .as_ref()
            .and_then(|elsewhere| elsewhere.target(target))
            .and_then(|target| target.template.as_deref())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ElsewhereFrontMatter {
    pub excerpt: Option<String>,
    pub mastodon: Option<ElsewhereTargetOverride>,
    pub bluesky: Option<ElsewhereTargetOverride>,
    pub markdown: Option<ElsewhereTargetOverride>,
    pub reddit: Option<RedditTargetOverride>,
}

impl ElsewhereFrontMatter {
    pub fn target(&self, target: &str) -> Option<&ElsewhereTargetOverride> {
        match target {
            "mastodon" => self.mastodon.as_ref(),
            "bluesky" => self.bluesky.as_ref(),
            "markdown" => self.markdown.as_ref(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ElsewhereTargetOverride {
    pub template: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RedditTargetOverride {
    pub subreddit: Option<String>,
    pub kind: Option<RedditPostKind>,
    pub title_template: Option<String>,
    pub body_template: Option<String>,
    pub comment_template: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_first_paragraph_from_markdown() {
        let markdown = r#"

First paragraph.

Second paragraph.
"#;

        assert_eq!(
            CanonicalPost::first_paragraph_from_markdown(markdown).as_deref(),
            Some("First paragraph.")
        );
    }

    #[test]
    fn editorial_excerpt_prefers_elsewhere_excerpt() {
        let mut post = test_post("Example");
        post.description = Some("Description fallback.".to_string());
        post.first_paragraph = Some("First paragraph fallback.".to_string());
        post.elsewhere = Some(ElsewhereFrontMatter {
            excerpt: Some("Custom excerpt.".to_string()),
            mastodon: None,
            bluesky: None,
            markdown: None,
            reddit: None,
        });

        assert_eq!(post.editorial_excerpt(), "Custom excerpt.");
    }

    #[test]
    fn editorial_excerpt_falls_back_to_description() {
        let mut post = test_post("Example");
        post.description = Some("Description fallback.".to_string());
        post.first_paragraph = Some("First paragraph fallback.".to_string());

        assert_eq!(post.editorial_excerpt(), "Description fallback.");
    }

    #[test]
    fn editorial_excerpt_falls_back_to_first_paragraph() {
        let mut post = test_post("Example");
        post.first_paragraph = Some("First paragraph fallback.".to_string());

        assert_eq!(post.editorial_excerpt(), "First paragraph fallback.");
    }

    fn test_post(title: &str) -> CanonicalPost {
        CanonicalPost {
            title: title.to_string(),
            description: None,
            date: None,
            tags: Vec::new(),
            canonical_url: None,
            body_markdown: String::new(),
            first_paragraph: None,
            slug: None,
            elsewhere: None,
            path: None,
            draft: false,
        }
    }
}
