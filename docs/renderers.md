# Renderers

Renderers turn a canonical post from your site into a publishing draft for another place.

Elsewhere does not publish directly to any platform. It prepares output for review, editing, and manual posting.

```text
canonical post
  -> renderer
  -> publishing draft
```

A renderer may produce plain text, Markdown, or a structured draft depending on the target.

## Supported renderers

Elsewhere currently supports:

```text
mastodon
bluesky
reddit
markdown
```

Render one target:

```sh
elsewhere render mastodon content/writing/example-post.md
```

Render all targets:

```sh
elsewhere render all content/writing/example-post.md
```

Preview all targets:

```sh
elsewhere plan content/writing/example-post.md
```

Emit a machine-readable plan:

```sh
elsewhere plan --json content/writing/example-post.md
```

## Renderer types

Not every renderer has the same shape.

Mastodon and Bluesky are short-form text targets.

Markdown is a long-form publishing draft.

Reddit is a structured draft with separate fields such as subreddit, submission kind, title, URL, body, and suggested first comment.

Elsewhere should reflect the shape of the destination instead of pretending every platform is just one text box.

## Common template behaviour

Most renderers use templates.

A template can reference fields from the canonical post:

```text
{title}
{description}
{excerpt}
{url}
```

Example:

```toml
[mastodon]
max_chars = 500
template = """
{excerpt}

New post: {title}
{url}
"""
```

Renderer templates can be configured globally in `elsewhere.toml`.

Per-post overrides can replace a site-level renderer template.

For Zola:

```toml
[extra.elsewhere.mastodon]
template = """
A custom version for Mastodon.

{excerpt}

{url}
"""
```

For generic Markdown:

```toml
[elsewhere.mastodon]
template = """
A custom version for Mastodon.

{excerpt}

{url}
"""
```

The per-post template wins over the site-level template.

## Excerpts

The `{excerpt}` variable is editorially selected.

Elsewhere resolves it in this order:

```text
1. per-post Elsewhere excerpt
2. description
3. first paragraph
4. title
```

For Zola:

```toml
[extra.elsewhere]
excerpt = "A custom syndication excerpt."
```

For generic Markdown:

```toml
[elsewhere]
excerpt = "A custom syndication excerpt."
```

A first paragraph is often a good excerpt. Sometimes it isn't. Use an explicit excerpt when a post needs a different shape somewhere else.

## Mastodon

The Mastodon renderer produces a short plain-text draft.

Example command:

```sh
elsewhere render mastodon content/writing/example-post.md
```

Example configuration:

```toml
[mastodon]
max_chars = 500
template = """
{excerpt}

New post: {title}
{url}

{hashtags}"""
```

The Mastodon renderer checks output against the configured character limit.

If the rendered draft exceeds the configured limit, Elsewhere warns.

Example output:

```text
This is a deliberately small example post used to test syndication drafts.

New post: A Tiny Example Post
https://example.com/writing/example-post/

#example #markdown #posse
```

## Bluesky

The Bluesky renderer produces a short plain-text draft.

Example command:

```sh
elsewhere render bluesky content/writing/example-post.md
```

Example configuration:

```toml
[bluesky]
max_chars = 300
template = """
New post: {title}

{excerpt}

{url}"""
```

The Bluesky renderer checks output against the configured character limit.

If the rendered draft exceeds the configured limit, Elsewhere warns.

Example output:

```text
New post: A Tiny Example Post

This is a deliberately small example post used to test syndication drafts.

https://example.com/writing/example-post/
```

## Reddit

The Reddit renderer produces a structured posting draft.

Reddit is different from Mastodon and Bluesky. A Reddit submission is not just a short text box. It has a destination community, a submission kind, a title, and either a URL or a body. It may also have a suggested first comment.

Elsewhere supports two Reddit submission kinds:

```text
link
selfpost
```

A link submission points to the canonical URL.

Example configuration:

```toml
[reddit]
kind = "link"
subreddit = "example"
title_template = "{title}"
comment_template = """
{excerpt}

Originally published here:
{url}
"""
```

A self post includes a body.

Example configuration:

```toml
[reddit]
kind = "selfpost"
subreddit = "example"
title_template = "{title}"
body_template = """
{excerpt}

{url}
"""
```

The `subreddit` value may be written with or without `r/`.

These are equivalent:

```toml
subreddit = "example"
```

```toml
subreddit = "r/example"
```

Per-post Reddit overrides are supported.

For Zola:

```toml
[extra.elsewhere.reddit]
subreddit = "example"
kind = "link"
title_template = "A Tiny Example Post"
comment_template = """
This is the suggested first comment for the example Reddit draft.

{excerpt}

Source:
{url}
"""
```

For generic Markdown:

```toml
[elsewhere.reddit]
subreddit = "example"
kind = "link"
title_template = "A Tiny Example Post"
comment_template = """
This is the suggested first comment for the example Reddit draft.

{excerpt}

Source:
{url}
"""
```

Example rendered draft:

```text
Subreddit: r/example
Kind: link

Title:
A Tiny Example Post

URL:
https://example.com/writing/example-post/

Suggested first comment:
This is the suggested first comment for the example Reddit draft.

This is a deliberately small example post used to test syndication drafts.

Source:
https://example.com/writing/example-post/
```

Elsewhere does not post to Reddit.

You still need to check the subreddit rules, decide whether the post belongs there, and submit it yourself.

## Markdown

The Markdown renderer produces a long-form publishing draft.

Example command:

```sh
elsewhere render markdown content/writing/example-post.md > post.md
```

Example configuration:

```toml
[markdown]
template = """
# {title}

_{description}_

{body}

{canonical_phrase}
{url}"""
```

The Markdown renderer is useful for Markdown-friendly publishing workflows, including tools such as Ghost, WriteFreely, Bear Blog, DEV.to, Hashnode, or any editor that accepts Markdown cleanly.

It is not the same as copying the source Markdown file.

Elsewhere reads the Markdown used by your static site. That source file may contain front matter, taxonomies, draft flags, aliases, site-specific metadata, shortcodes, and Elsewhere overrides.

The Markdown renderer outputs a publishing draft shaped by a template.

```text
site Markdown in
publishing Markdown out
```

Example rendered draft:

```markdown
# A Tiny Example Post

_A short demonstration post for Elsewhere's example project._

This is a tiny example post.

It exists so Elsewhere has something safe, boring, and copy-pastable to render during tests, demos, and documentation updates.

Originally published on my website:
https://example.com/writing/example-post/
```

Substack is more complicated because its editor is not simply “paste Markdown and go.” Elsewhere can prepare a Markdown draft, but it does not currently generate a rich-text clipboard payload or publish directly to Substack.

## Warnings

Renderers may produce warnings.

Common warnings include:

```text
post is marked as draft
rendered output exceeds the configured character limit
required renderer configuration is missing
```

Warnings appear in `plan` output and on stderr during `render`.

Example:

```text
Mastodon
  Status: warning
  Length: 534 / 500
  Warning: mastodon render is 534 characters. Configured limit is 500.
```

Warnings are meant to support review. They do not replace judgement.

## Adding a renderer

A renderer should prepare a publishing draft, not publish directly.

A good renderer should:

```text
- use the canonical post model
- preserve the canonical URL when appropriate
- support templates when useful
- produce clear warnings
- avoid hiding platform-specific quirks
- keep platform automation separate from rendering
```

Some targets are simple text templates.

Some targets are structured artifacts.

The renderer should match the destination’s actual shape.
