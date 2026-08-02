# Sources

A source tells Elsewhere how to read a post from your static site.

Elsewhere turns every supported source into the same canonical post before planning or rendering begins:

```
source file
    -> source adapter
    -> canonical post
    -> renderers
```

Renderers do not read Zola or generic Markdown front matter directly. They receive the canonical post produced by the selected source adapter.

This separation allows the same Mastodon, Bluesky, Reddit, and Markdown renderers to work with different static-site generators.

## Supported sources

Elsewhere currently supports:

- `generic` for Markdown files with TOML front matter; and
- `zola` for posts in a Zola site.

Select the source in `elsewhere.toml`:

```toml
source = "generic"
```

or:

```toml
source = "zola"
```

Both current source adapters use the same Markdown and front-matter parser. They differ primarily in how Elsewhere discovers the site URL and derives the post's canonical URL.

See:

* [Using Generic Markdown](generic-markdown.md); and
* [Using Elsewhere with Zola](zola.md).

## Loading a post

When you pass a post to `plan` or `render`, Elsewhere:

1. confirms that the post file exists
2. finds the applicable `elsewhere.toml`
3. resolves the post path relative to the site root
4. determines the site's public base URL
5. reads and parses the source post
6. converts it into a canonical post
7. derives a canonical URL if the post does not already provide one.

For example:

```sh
elsewhere plan content/writing/example-post.md
```

The resulting canonical post is then passed to every configured renderer.

Elsewhere only reads the source post. Planning and rendering do not modify it.

## Markdown format

Both current source adapters expect a Markdown file with TOML front matter.

The file must begin with `+++`:

```md
+++
title = "Example Post"
description = "A small example."
date = 2026-07-29
tags = ["indieweb", "writing"]
+++

This is the first paragraph.

This is the rest of the post.
```

The closing `+++` separates the front matter from the Markdown body.

Elsewhere does not currently support YAML front matter:

```md
---
title: Example Post
---
```

A file using `---` is rejected rather than interpreted as TOML.

The opening delimiter must be the first line of the file. The front matter must also have a closing delimiter.

## The canonical post

A canonical post is Elsewhere's source-neutral representation of a post.

It contains the following fields.

| Field             |        Required | Description                            |
| ----------------- | --------------: | -------------------------------------- |
| `title`           |             Yes | The title of the post                  |
| `description`     |              No | A short description or summary         |
| `date`            |              No | The publication date                   |
| `tags`            |              No | A list of tags                         |
| `canonical_url`   |              No | The public URL of the original post    |
| `body_markdown`   |             Yes | The Markdown body without front matter |
| `first_paragraph` |         Derived | The first non-empty block of the body  |
| `slug`            | Usually derived | The post slug                          |
| `path`            |              No | An explicit public path                |
| `draft`           |              No | Whether the source post is a draft     |
| `elsewhere`       |              No | Elsewhere-specific editorial metadata  |

Only `title` is required in the front matter.

The Markdown body may be empty, although renderers using `{body}`, `{first_paragraph}`, or `{excerpt}` may produce sparse output or fall back to other fields.

## Title

`title` is required and must be a string:

```toml
title = "The Website Is the Source of Truth"
```

A missing title is an error.

The title is available to templates as:

```
{title}
```

## Description

`description` is optional and must be a string:

```toml
description = "On writing locally and publishing elsewhere."
```

It is available to templates as:

```
{description}
```

The description is also one of the fallback values used to construct `{excerpt}`.

A template that requests `{description}` fails when the post does not provide one.

## Date

`date` is optional.

Elsewhere accepts either a string:

```toml
date = "2026-07-29"
```

or a TOML date or datetime:

```toml
date = 2026-07-29
```

```toml
date = 2026-07-29T09:30:00-07:00
```

Elsewhere preserves the parsed value as text for rendering. It does not currently reformat dates for different destinations.

The date is available to templates as:

```
{date}
```

A template that requests `{date}` fails when the post does not provide one.

## Tags

Tags may be provided as a top-level array:

```toml
tags = ["indieweb", "posse", "static-sites"]
```

Zola-style taxonomy tags are also supported:

```toml
[taxonomies]
tags = ["indieweb", "posse", "static-sites"]
```

When a non-empty `taxonomies.tags` array is present, Elsewhere uses it instead of the top-level `tags` field.

If no tags are present, the canonical post contains an empty list.

Tags are available to templates as:

```
{tags}
```

The template value joins tags with `, `:

```
indieweb, posse, static-sites
```

Elsewhere does not currently convert tags into platform-specific hashtags.

## Markdown body

Everything after the closing front-matter delimiter becomes the canonical Markdown body.

Given:

```md
+++
title = "Example"
+++

First paragraph.

Second paragraph.
```

`body_markdown` contains:

```md
First paragraph.

Second paragraph.
```

The original front matter is not included.

The body is available to templates through either of these equivalent variables:

```
{body}
{body_markdown}
```

Elsewhere preserves the Markdown source. It does not render the body to HTML before passing it to renderers.

## First paragraph

Elsewhere derives `first_paragraph` from the Markdown body.

The current parser divides the body at blank lines, trims each resulting block, and selects the first non-empty block.

For example:

```md
This is the opening paragraph.

This is the second paragraph.
```

produces:

```
This is the opening paragraph.
```

This is a deliberately small Markdown heuristic. Elsewhere does not parse the document into a full Markdown syntax tree when selecting the first paragraph.

As a result, the first non-empty block may be a heading, block quote, list, or fenced code block when one appears before the opening prose.

For example:

```md
## Introduction

This is the first prose paragraph.
```

produces:

```
## Introduction
```

The derived value is available to templates as:

```
{first_paragraph}
```

A template that requests `{first_paragraph}` fails when the body contains no non-empty block.

## Slug

`slug` is optional:

```toml
slug = "website-as-source-of-truth"
```

When it is not provided, Elsewhere attempts to infer it from the post's filename.

For:

```
content/writing/example-post.md
```

the inferred slug is:

```
example-post
```

An explicit slug takes precedence over the filename.

The slug is available to templates as:

```
{slug}
```

It may also be used when deriving the canonical URL.

A template that requests `{slug}` fails when Elsewhere cannot determine one.

## Path

`path` is an optional explicit public path:

```toml
path = "/essays/example-post/"
```

It is primarily used by the Zola source when deriving the canonical URL.

The path is joined with the effective site URL:

```
https://example.com/essays/example-post/
```

`path` is not currently exposed as a template variable.

An explicit `canonical_url` still takes precedence over `path`.

## Draft status

`draft` is an optional boolean:

```toml
draft = true
```

It defaults to `false`.

Draft status does not prevent Elsewhere from planning or rendering the post. Instead, Elsewhere includes a warning in the plan and rendered diagnostics.

This allows you to inspect publishing drafts before the source post is public without silently treating the post as published.

## Canonical URLs

The canonical URL identifies the original post on your website.

Renderers commonly expose it through:

```
{url}
```

or:

```
{canonical_url}
```

These variables are equivalent.

Elsewhere determines the canonical URL in two stages:

1. select the effective site URL
2. select or derive the post path

### Effective site URL

A top-level `site_url` in `elsewhere.toml` takes precedence for every source:

```toml
site_url = "https://example.com"
```

When `site_url` is absent:

- the Zola source reads `base_url` from `zola.toml`;
- the generic source returns an error because it has no site configuration from which to infer the URL.

### URL precedence

An explicit `canonical_url` in the post always wins:

```toml
canonical_url = "https://example.com/special/example/"
```

When it is absent, Elsewhere delegates URL construction to the selected source.

For generic Markdown, Elsewhere applies the configured `url_pattern` to the post's slug.

For Zola, Elsewhere uses this order:

1. explicit `path`
2. the post's location beneath `content_dir`, when path-based section URLs are enabled
3. the generic slug pattern, when path-based section URLs are disabled.

Within a path derived from the content tree, an explicit slug replaces the filename-derived final segment.

See [Configuration](configuration.md), [Using Generic Markdown](generic-markdown.md), and [Using Elsewhere with Zola](zola.md) for source-specific examples.

## Editorial metadata

Posts can provide metadata used only by Elsewhere.

This metadata does not replace the canonical post. It supplies editorial choices for a particular publishing workflow.

The shared editorial model supports:

- an explicit excerpt
- a Mastodon template override
- a Bluesky template override
- a Markdown template override
- structured Reddit overrides

Generic Markdown normally places this metadata under:

```toml
[elsewhere]
```

Zola normally places it under:

```toml
[extra.elsewhere]
```

The parser recognizes both forms for either source.

When both forms are present in the same post, `[extra.elsewhere]` takes precedence over `[elsewhere]`. The two tables are not merged.

See [Renderers](renderers.md) for the supported per-target fields.

## Editorial excerpts

Renderers use the canonical post's editorial excerpt through:

```
{excerpt}
```

Elsewhere selects the first available value in this order:

1. `elsewhere.excerpt`
2. `description`
3. `first_paragraph`
4. `title`

For example:

```toml
title = "A Long and Specific Title"
description = "The ordinary site description."

[elsewhere]
excerpt = "A shorter introduction written for syndication."
```

produces:

```
A shorter introduction written for syndication.
```

An explicit excerpt is useful when the site description is appropriate for metadata but not for a social post.

Because the title is the final fallback, `{excerpt}` always produces a value for a valid post.

## Source-specific metadata

Elsewhere reads a deliberately small subset of front matter into the canonical post.

The current canonical model does not include fields such as:

- authors
- aliases
- templates
- updated dates
- language
- weights
- taxonomies other than tags
- arbitrary values under `extra`

These fields may remain useful to your static-site generator, but Elsewhere does not currently expose them to renderers.

The exception is Elsewhere's own metadata under `[elsewhere]` or `[extra.elsewhere]`.

Unknown front-matter fields are otherwise left unused.

## Invalid source files

Elsewhere reports an error when:

- the post file does not exist
- no applicable `elsewhere.toml` can be found
- the file does not begin with TOML front matter
- YAML front matter is used
- the closing `+++` delimiter is missing
- the front matter is not valid TOML
- the front-matter root is not a table
- `title` is missing
- a recognized field has the wrong type
- generic Markdown has no configured `site_url`
- the selected source's site configuration cannot be read

Errors occur before rendering begins.

A renderer may also fail later when its template requests an optional canonical field that the source post does not provide.

Use:

```sh
elsewhere plan path/to/post.md
```

to inspect the canonical title, URL, tags, draft status, renderer warnings, and renderer-specific errors before producing publishing drafts.

## Adding source adapters

The canonical post is the boundary between source adapters and renderers.

A future source adapter is responsible for:

1. reading a source file
2. mapping its metadata into the canonical fields
3. preserving its body as Markdown
4. supplying or deriving any source-specific metadata
5. participating in canonical URL construction

It should not contain Mastodon-, Bluesky-, Reddit-, or export-specific rendering logic.

That belongs on the other side of the canonical post boundary.
