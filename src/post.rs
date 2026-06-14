#[derive(Debug, Clone)]
#[allow(dead_code)] // We'll need these later for rendering
pub struct CanonicalPost {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub canonical_url: Option<String>,
    pub body_markdown: String,
    pub first_paragraph: Option<String>,
    pub slug: Option<String>,

    // Zola/font-matter page metadata
    pub path: Option<String>,
    pub draft: bool,
    pub aliases: Vec<String>,
}

impl CanonicalPost {
    pub fn first_paragraph_from_markdown(markdown: &str) -> Option<String> {
        markdown
            .split("\n\n")
            .map(str::trim)
            .find(|paragraph| !paragraph.is_empty())
            .map(ToOwned::to_owned)
    }
}
