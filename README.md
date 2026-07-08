# Elsewhere

Elsewhere is a local POSSE CLI for static-site writers.

It treats your website as the canonical source of your writing and renders platform-specific versions for other places.

Your website is the home. Platforms are edges.

## Why?

Publishing on the web often means copying the same post into several different places.

Mastodon wants one shape. Bluesky wants another. Long-form publishing tools often want Markdown, HTML, or some editor-specific paste format. Each platform has its own limits, templates, habits, and annoying vibes.

Elsewhere keeps the original on your site and renders copies from there.

It does not make platforms the source of truth.

## Install

Once published to crates.io:

```sh
cargo install elsewhere
```

During development, install from a local checkout:

```sh
cargo install --path .
```

Then run:

```sh
elsewhere --help
```

## Quick start

Create an `elsewhere.toml` file in the root of your static site:

```sh
elsewhere init
```

Preview how Elsewhere understands a post:

```sh
elsewhere plan content/writing/my-post.md
```

Render a post for Mastodon:

```sh
elsewhere render mastodon content/writing/my-post.md
```

Render all supported targets:

```sh
elsewhere render all content/writing/my-post.md
```

Export a long-form Markdown draft:

```sh
elsewhere render markdown content/writing/my-post.md > post.md
```

## Supported sources

Elsewhere currently supports:

* Generic Markdown sites
* Zola sites

For Zola, Elsewhere can read `base_url` from the site’s existing `config.toml`.

## Supported renderers

Elsewhere currently renders:

* Mastodon
* Bluesky
* Markdown

The Markdown renderer produces a long-form publishing draft. It is suitable for Markdown-friendly publishing workflows, including tools such as Ghost, WriteFreely, Bear Blog, DEV.to, Hashnode, or any editor that accepts Markdown cleanly.

Elsewhere does not publish directly to any platform. It prepares the output; you decide where it goes.

## Why export Markdown if Elsewhere already reads Markdown?

Elsewhere reads the Markdown used by your static site. That file is the canonical source.

The Markdown export is not the same file copied somewhere else. It is a rendered draft for another publishing context.

Your source post may contain front matter, taxonomies, draft flags, aliases, site-specific paths, shortcodes, internal metadata, and Elsewhere editorial overrides. The exported Markdown is shaped for publication elsewhere: title, description, body, canonical link, and whatever template you configured for that target.

In other words:

```text
site Markdown in
publishing Markdown out
```

Elsewhere keeps the source file canonical, then produces a cleaner or differently shaped Markdown draft for tools that accept Markdown well.

## Example

Given a post like this:

```toml
+++
title = "The Boos Made Sense"
description = "AI, graduation speeches, and the broken promise of technological disruption."
date = "2026-06-14"

[taxonomies]
tags = ["ai", "platforms", "labour"]

[extra.elsewhere]
excerpt = "A graduation ceremony is a strange place to sell uncertainty."

[extra.elsewhere.mastodon]
template = """
The boos made sense.

{excerpt}

{url}

{hashtags}
"""
+++

The boos made sense.

A graduation ceremony is a strange place to sell uncertainty.
```

Elsewhere can plan all rendered outputs:

```sh
elsewhere plan content/writing/the-boos-made-sense.md
```

Example output:

```text
Elsewhere plan

Canonical
  Title: The Boos Made Sense
  URL:   https://example.com/writing/the-boos-made-sense/
  Tags:  ai, platforms, labour

Mastodon
  Status: ready
  Length: 113 / 500

  The boos made sense.

  A graduation ceremony is a strange place to sell uncertainty.

  https://example.com/writing/the-boos-made-sense/

  #ai #platforms #labour

Bluesky
  Status: ready
  Length: 150 / 300

  New essay: The Boos Made Sense

  AI, graduation speeches, and the broken promise of technological disruption.

  https://example.com/writing/the-boos-made-sense/

Markdown
  Status: ready
  Length: 276
  Output: use `elsewhere render markdown content/writing/the-boos-made-sense.md > markdown.md`
```

## Configuration

A small Zola configuration looks like this:

```toml
content_dir = "content"
source = "zola"

[defaults]
canonical_phrase = "Originally published on my website:"

[zola]
section_url_from_path = true
```

A generic Markdown configuration can provide its own URL pattern:

```toml
site_url = "https://example.com"
content_dir = "content"
source = "generic"

[generic]
url_pattern = "/writing/{slug}/"
```

Renderers can be customized with templates:

```toml
[mastodon]
max_chars = 500
template = """
{excerpt}

New essay: {title}
{url}

{hashtags}"""
```

The Markdown renderer can also be customized:

```toml
[markdown]
template = """
# {title}

_{description}_

{body}

{canonical_phrase}
{url}"""
```

Per-post editorial overrides are also supported.

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

For generic Markdown files, use `[elsewhere]`:

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

## Philosophy

Elsewhere is deliberately small.

It has:

* no account
* no hosted service
* no dashboard
* no analytics
* no scheduling
* no automatic posting
* no platform as the source of truth

It reads posts from your static site and renders platform-specific output.

That is all.

The restraint is the point.

## Roadmap

Possible future work:

* Hugo support
* Eleventy support
* more render targets
* HTML export for rich-text editors
* clipboard support for editors that accept `text/html`
* posting APIs
* syndication tracking
* `u-syndication` backlinks
* better plain-text extraction from Markdown

Direct publishing may happen later, but only after the rendering workflow is solid.

## License
Elsewhere is published under the GNU General Public License, version 3 or later.

See `COPYING` for details.
