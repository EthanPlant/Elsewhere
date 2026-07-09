# Configuration

Elsewhere is configured with an `elsewhere.toml` file in the root of your static site.

The config tells Elsewhere where your content lives, how to derive canonical URLs, and how to render posts for each target.

## Minimal Zola config

```toml
content_dir = "content"
source = "zola"

[defaults]
canonical_phrase = "Originally published on my website:"

[zola]
section_url_from_path = true
```

For Zola sites, Elsewhere reads `base_url` from the site’s existing `config.toml`.

Example:

```toml
base_url = "https://example.com"
```

A post at:

```text
content/writing/example-post.md
```

will render with a canonical URL like:

```text
https://example.com/writing/example-post/
```

## Minimal generic Markdown config

```toml
site_url = "https://example.com"
content_dir = "content"
source = "generic"

[generic]
url_pattern = "/writing/{slug}/"
```

Generic Markdown sites must provide `site_url`, because Elsewhere cannot infer it from a static site generator config.

The `url_pattern` controls how canonical URLs are derived.

For example:

```toml
url_pattern = "/writing/{slug}/"
```

with:

```text
content/writing/example-post.md
```

produces:

```text
https://example.com/writing/example-post/
```

## Shared defaults

```toml
[defaults]
canonical_phrase = "Originally published on my website:"
```

`canonical_phrase` is available to templates as:

```text
{canonical_phrase}
```

It is usually used in long-form Markdown exports.

## Mastodon

```toml
[mastodon]
max_chars = 500
template = """
{excerpt}

New post: {title}
{url}

{hashtags}"""
```

`max_chars` controls the warning threshold used by `plan` and `render`.

The template controls the rendered Mastodon draft.

## Bluesky

```toml
[bluesky]
max_chars = 300
template = """
New post: {title}

{excerpt}

{url}"""
```

`max_chars` controls the warning threshold used by `plan` and `render`.

## Reddit

The Reddit renderer prepares a structured posting draft. It does not post to Reddit.

A link submission:

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

A self post:

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

Supported `kind` values:

```text
link
selfpost
```

The `subreddit` value may be written with or without `r/`.

For example, these are equivalent:

```toml
subreddit = "example"
```

```toml
subreddit = "r/example"
```

## Markdown

The Markdown renderer produces a long-form publishing draft.

```toml
[markdown]
template = """
# {title}

_{description}_

{body}

{canonical_phrase}
{url}"""
```

This is not the same as copying the source Markdown file.

Elsewhere reads the Markdown used by your static site. That file may contain front matter, taxonomies, draft flags, aliases, site-specific metadata, and Elsewhere overrides.

The Markdown renderer outputs a publishing draft shaped by a template.

```text
site Markdown in
publishing Markdown out
```

## Template variables

Renderer templates may use these variables:

```text
{title}
{description}
{excerpt}
{first_paragraph}
{url}
{canonical_url}
{date}
{slug}
{tags}
{body}
{body_markdown}
{canonical_phrase}
```

`{excerpt}` uses Elsewhere’s editorial fallback order:

```text
1. per-post Elsewhere excerpt
2. description
3. first paragraph
4. title
```

`{tags}` renders a comma-separated tag list.

`{hashtags}` renders tags as hashtags.

For example:

```toml
tags = ["example", "markdown", "posse"]
```

renders as:

```text
#example #markdown #posse
```

## Per-post overrides

Elsewhere supports per-post editorial overrides.

For Zola posts, use `[extra.elsewhere]`:

```toml
[extra.elsewhere]
excerpt = "A custom excerpt for syndication."

[extra.elsewhere.mastodon]
template = """
A custom Mastodon version.

{excerpt}

{url}
"""
```

For generic Markdown posts, use `[elsewhere]`:

```toml
[elsewhere]
excerpt = "A custom excerpt for syndication."

[elsewhere.mastodon]
template = """
A custom Mastodon version.

{excerpt}

{url}
"""
```

Per-post platform templates override site-level renderer templates.

For example, this Mastodon override applies only to that post:

```toml
[extra.elsewhere.mastodon]
template = """
A tiny example appears.

{excerpt}

{url}
"""
```

Supported per-post target sections:

```text
mastodon
bluesky
reddit
markdown
```

## Zola front matter

Elsewhere supports common Zola front matter fields:

```toml
slug = "custom-slug"
path = "custom/path"
draft = true
```

It also reads Zola tags from taxonomies:

```toml
[taxonomies]
tags = ["example", "markdown", "posse"]
```

If `canonical_url` is set directly in front matter, it overrides any derived URL:

```toml
canonical_url = "https://example.com/custom-url/"
```

## Runnable example

A complete example project is available at:

```text
examples/zola
```

Run it with:

```sh
cd examples/zola
cargo run --manifest-path ../../Cargo.toml -- plan content/writing/example-post.md
cargo run --manifest-path ../../Cargo.toml -- render all content/writing/example-post.md
cargo run --manifest-path ../../Cargo.toml -- render markdown content/writing/example-post.md > example-post.md.out
```
