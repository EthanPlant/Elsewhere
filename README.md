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

For Zola, Elsewhere can read `base_url` from the site’s existing `zola.toml`.

## Supported renderers

Elsewhere currently renders:

* Mastodon
* Bluesky
* Reddit
* Markdown

Mastodon and Bluesky are short-form text targets.

Reddit is a structured publishing target. It can prepare a link submission or self post, including a title, subreddit, URL or body, and optional suggested first comment. It does not post to Reddit for you. You still need to check the destination community, read the rules, and decide whether posting there is actually a good idea.

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

```md
+++
title = "A Tiny Example Post"
description = "A short demonstration post for Elsewhere's example project."
date = "2026-01-15"

[taxonomies]
tags = ["example", "markdown", "posse"]

[extra.elsewhere]
excerpt = "This is a deliberately small example post used to test syndication drafts."

[extra.elsewhere.mastodon]
template = """
A tiny example appears.

{excerpt}

{url}
"""

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
+++

This is a tiny example post.

It exists so Elsewhere has something safe, boring, and copy-pastable to render during tests, demos, and documentation updates.

The post is intentionally simple. It has a title, description, tags, a custom excerpt, a Mastodon override, a Reddit override, and enough body text to demonstrate the Markdown renderer.
```

Elsewhere can plan all rendered outputs:

```sh
Reddit
elsewhere plan content/writing/example-post.md
```

Example output:

```text
Elsewhere plan

Canonical
  Title: A Tiny Example Post
  URL:   https://example.com/writing/example-post/
  Tags:  example, markdown, posse

Mastodon
  Status: ready
  Length: 143 / 500

  A tiny example appears.

  This is a deliberately small example post used to test syndication drafts.

  https://example.com/writing/example-post/

Bluesky
  Status: ready
  Length: 147 / 300

  New post: A Tiny Example Post

  This is a deliberately small example post used to test syndication drafts.
  https://example.com/writing/example-post/

Markdown
  Status: ready
  Length: 508
  Output: use `elsewhere render markdown content/writing/example-post.md > markdown.md`

Reddit
  Status: ready
  Length: 332

  Subreddit: r/example
  Kind: link

  Title:
  A Tiny Example Post

  URL:
  https://example.com/writing/example-post/

  Suggested first comment:
  This is a deliberately small example post used to test syndication drafts.

  Originally published here: https://example.com/writing/example-post/


  Reminder: check the subreddit rules before posting.
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

See `LICENSE` for details.
