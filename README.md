# Elsewhere

Elsewhere is a local POSSE CLI for static-site writers.

It treats your website as the canonical source of your writing and renders platform-specific publishing drafts for other places.

Your website is the home. Platforms are edges.

## Why?

Publishing on the web often means copying the same post into several different places.

Mastodon wants one shape. Bluesky wants another. Reddit has titles, communities, link posts, self posts, and rules. Long-form publishing tools often want Markdown, HTML, or some editor-specific paste format. Each platform has its own limits, templates, habits, and annoying vibes.

Elsewhere keeps the original on your site and renders drafts from there.

It does not make platforms the source of truth.

## Install

From crates.io:

```sh
cargo install elsewhere
```

From a local checkout:

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

Render a Reddit draft:

```sh
elsewhere render reddit content/writing/my-post.md
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
* Reddit
* Markdown

Mastodon and Bluesky are short-form text targets.

Reddit is a structured publishing target. It can prepare a link submission or self post, including a title, subreddit, URL or body, and optional suggested first comment. It does not post to Reddit for you.

Markdown produces a long-form publishing draft. It is suitable for Markdown-friendly publishing workflows, including tools such as Ghost, WriteFreely, Bear Blog, DEV.to, Hashnode, or any editor that accepts Markdown cleanly.

Elsewhere does not publish directly to any platform. It prepares the output; you decide where it goes.

## Example

Given a Zola post like this:

```toml
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
```

Elsewhere can plan all rendered outputs:

```sh
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
  Length: 145 / 500

  A tiny example appears.

  This is a deliberately small example post used to test syndication drafts.

  https://example.com/writing/example-post/

Bluesky
  Status: ready
  Length: 143 / 300

  New post: A Tiny Example Post

  This is a deliberately small example post used to test syndication drafts.

  https://example.com/writing/example-post/

Reddit
  Status: ready
  Length: 252

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

Markdown
  Status: ready
  Length: 390
  Output: use `elsewhere render markdown content/writing/example-post.md > markdown.md`
```

A complete runnable example is available in [`examples/zola`](examples/zola).

Try it from the repository root:

```sh
cd examples/zola
cargo run --manifest-path ../../Cargo.toml -- plan content/writing/example-post.md
cargo run --manifest-path ../../Cargo.toml -- render all content/writing/example-post.md
cargo run --manifest-path ../../Cargo.toml -- render markdown content/writing/example-post.md > example-post.md.out
```

## Configuration

Elsewhere uses an `elsewhere.toml` file in the root of your static site.

A small Zola configuration looks like this:

```toml
content_dir = "content"
source = "zola"

[defaults]
canonical_phrase = "Originally published on my website:"

[zola]
section_url_from_path = true
```

A small generic Markdown configuration looks like this:

```toml
site_url = "https://example.com"
content_dir = "content"
source = "generic"

[generic]
url_pattern = "/writing/{slug}/"
```

Renderer templates, Reddit options, Markdown export, and per-post overrides are documented in [`docs/configuration.md`](docs/configuration.md).

Renderer behaviour is documented in [`docs/renderers.md`](docs/renderers.md).

## Why export Markdown if Elsewhere already reads Markdown?

Elsewhere reads the Markdown used by your static site. That file is the canonical source.

The Markdown export is not the same file copied somewhere else. It is a rendered draft for another publishing context.

Your source post may contain front matter, Zola taxonomies, draft flags, aliases, site-specific paths, shortcodes, internal metadata, and Elsewhere editorial overrides. The exported Markdown is shaped for publication elsewhere: title, description, body, canonical link, and whatever template you configured for that target.

In other words:

```text
site Markdown in
publishing Markdown out
```

Elsewhere keeps the source file canonical, then produces a cleaner or differently shaped Markdown draft for tools that accept Markdown well.

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

It reads posts from your static site and renders platform-specific publishing drafts.

That is all. The restraint is the point.

Elsewhere keeps the final decision with the human. It can prepare drafts, show previews, structure platform-specific output, and warn about obvious problems. But it does not post for you. That is deliberate. The tool should support editorial judgement, not replace it.

The intended workflow is:

```text
plan
review
render
edit
post manually
```

That friction is useful. It makes the writer look at the output before publishing. It leaves room for taste, context, community norms, and the simple question of whether something should be posted somewhere at all.

This matters especially for places like Reddit, where the question is not merely “does this fit the template?” but “does this belong in this community?” Elsewhere can prepare a Reddit draft. It cannot read the room for you. It cannot turn syndication into permission.

Any tool that makes publishing easier can be misused. Elsewhere cannot fully prevent that. Someone can always wrap a CLI in a shell script and make everyone’s day worse. But Elsewhere should not make that the natural path. It should not be a spray-and-pray spam machine. It should make thoughtful syndication easier while keeping the final act of publishing in human hands.

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

Elsewhere is licensed under the GNU General Public License, version 3 or later.

See [`LICENSE`](LICENSE) for details.
