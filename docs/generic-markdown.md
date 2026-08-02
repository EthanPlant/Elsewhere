# Using Generic Markdown

Elsewhere can read Markdown posts without a dedicated static-site-generator adapter.

The generic source is intended for sites that:

- store posts as Markdown
- use TOML front matter
- have a predictable URL pattern
- do not need Elsewhere to understand generator-specific routing rules

Elsewhere reads the source file, maps its metadata into a canonical post, and derives its public URL from a configured site URL and slug pattern.

It does not run your static-site generator, inspect its configuration, or verify the resulting URL against the built site.

## Initialize Elsewhere

Run the following command from the root of your static site:

```sh
elsewhere init --source generic
```

Generic Markdown is the default source, so this is equivalent:

```sh
elsewhere init
```

Elsewhere creates an `elsewhere.toml` file in the current directory.

A minimal generic configuration looks like this:

```toml
site_url = "https://example.com"
content_dir = "content"
source = "generic"

[defaults]
canonical_phrase = "Originally published on my website:"

[generic]
url_pattern = "/writing/{slug}/"
```

Renderer tables are optional. When they are absent, Elsewhere uses its built-in renderer defaults.

See [Configuration](configuration.md) for the complete `elsewhere.toml` reference.

## Project layout

A generic Markdown site might look like this:

```
my-site/
├── elsewhere.toml
└── content/
    └── writing/
        └── example-post.md
```

The directory containing `elsewhere.toml` becomes the site root.

Elsewhere searches for `elsewhere.toml` starting from the post's directory and walking upward through its parent directories.

For example:

```sh
elsewhere plan content/writing/example-post.md
```

finds the configuration at the root of `my-site`.

The post path may also be absolute.

## A minimal post

Generic Markdown posts must use TOML front matter delimited by `+++`.

A minimal post looks like this:

```md
+++
title = "A Tiny Example Post"
+++

This is a tiny example post.
```

Plan it with:

```sh
elsewhere plan content/writing/example-post.md
```

Render a Mastodon draft with:

```sh
elsewhere render mastodon content/writing/example-post.md
```

Render every current target with:

```sh
elsewhere render all content/writing/example-post.md
```

## Front-matter format

The opening `+++` must be the first line of the file:

```md
+++
title = "Example Post"
+++

This is the body.
```

The closing `+++` separates the front matter from the Markdown body.

Elsewhere does not currently support YAML front matter:

```md
---
title: Example Post
---
```

A file beginning with `---` is rejected rather than interpreted as TOML.

Elsewhere also reports an error when:

- the opening front matter is missing
- the closing delimiter is missing
- the front matter is not valid TOML
- the front-matter root is not a table
- `title` is absent
- a recognized field has the wrong type

## Supported front matter

Elsewhere reads the following fields:

| Field             | Required | Description                                     |
| ----------------- | -------: | ----------------------------------------------- |
| `title`           |      Yes | Post title                                      |
| `description`     |       No | Short description or summary                    |
| `date`            |       No | Publication date or datetime                    |
| `tags`            |       No | List of tags                                    |
| `taxonomies.tags` |       No | Alternative Zola-style tag list                 |
| `slug`            |       No | Explicit URL slug                               |
| `canonical_url`   |       No | Complete canonical URL                          |
| `path`            |       No | Parsed, but not used for generic URL derivation |
| `draft`           |       No | Whether the source post is a draft              |
| `elsewhere`       |       No | Elsewhere-specific editorial metadata           |
| `extra.elsewhere` |       No | Alternative Zola-style Elsewhere metadata       |

Other front-matter fields are ignored by Elsewhere.

They remain available to whatever system builds your site, but they are not exposed to Elsewhere templates.

## Site URL

Generic Markdown requires a public `site_url` in `elsewhere.toml`:

```toml
site_url = "https://example.com"
```

Unlike the Zola source, the generic adapter has no generator configuration from which to discover this value.

If `site_url` is absent, Elsewhere cannot derive canonical URLs and reports an error.

Use the public origin of the canonical site. Do not use a local development or staging URL unless that is deliberately where the original post will live.

Elsewhere removes a trailing slash from `site_url` before joining it with the derived path, so both forms are accepted:

```toml
site_url = "https://example.com"
```

```toml
site_url = "https://example.com/"
```

A site URL may include a path prefix:

```toml
site_url = "https://example.com/blog"
```

With a URL pattern of `/writing/{slug}/`, this produces URLs beneath:

```
https://example.com/blog/writing/
```

## Content directory

The shared configuration contains a `content_dir` field:

```toml
content_dir = "content"
```

At this revision, the generic source does not use `content_dir` to construct canonical URLs.

It also does not require the post to be located beneath that directory. The post is read from the path passed to the command.

For generic Markdown, the canonical URL is derived from:

1. `site_url`
2. `generic.url_pattern`
3. the post slug

`content_dir` still describes the site's intended layout and keeps the configuration consistent with source adapters that do use the content tree.

## URL pattern

Configure generic URL construction under `[generic]`:

```toml
[generic]
url_pattern = "/writing/{slug}/"
```

The default pattern is:

```
/writing/{slug}/
```

The supported placeholder is:

```
{slug}
```

Elsewhere replaces every occurrence of `{slug}` with the post's effective slug, then joins the resulting path to `site_url`.

For example:

```toml
site_url = "https://example.com"

[generic]
url_pattern = "/notes/{slug}/"
```

with:

```toml
slug = "small-web"
```

produces:

```
https://example.com/notes/small-web/
```

Elsewhere removes leading slashes from the pattern while joining it to the site URL. Whether the final URL has a trailing slash depends on the pattern itself.

These patterns produce different URLs:

```toml
url_pattern = "/notes/{slug}/"
```

```text
https://example.com/notes/small-web/
```

```toml
url_pattern = "/notes/{slug}.html"
```

```text
https://example.com/notes/small-web.html
```

```toml
url_pattern = "/{slug}"
```

```text
https://example.com/small-web
```

Elsewhere does not interpret static-site-generator permalink syntax. Only `{slug}` has defined meaning.

For example, this does not substitute the date:

```toml
url_pattern = "/{date}/{slug}/"
```

The resulting URL still contains the literal text `{date}`.

## Canonical URL precedence

For generic Markdown, Elsewhere determines the canonical URL in this order:

1. an explicit `canonical_url` in the post
2. `site_url` combined with `generic.url_pattern` and the effective slug

An explicit canonical URL takes precedence over all derived values:

```toml
canonical_url = "https://example.com/essays/a-special-address/"
```

Elsewhere uses this value as written.

Use `canonical_url` when:

- the public route does not follow one site-wide slug pattern
- the post is published outside the usual content hierarchy
- the site uses dated or nested permalinks Elsewhere cannot derive
- an external build step controls the route
- one post needs a historical or manually assigned URL

## Slugs

A post can provide an explicit slug:

```toml
slug = "website-as-source-of-truth"
```

When `slug` is absent, Elsewhere infers it from the source filename.

For:

```
content/writing/example-post.md
```

the inferred slug is:

```
example-post
```

The filename extension is removed.

Given:

```toml
[generic]
url_pattern = "/writing/{slug}/"
```

Elsewhere produces:

```
https://example.com/writing/example-post/
```

An explicit slug replaces the filename-derived value:

```md
+++
title = "Example Post"
slug = "a-better-address"
+++
```

produces:

```
https://example.com/writing/a-better-address/
```

The slug is also available to renderer templates as:

```
{slug}
```

## Directories do not affect generic URLs

The generic adapter does not include the post's directory hierarchy in its canonical URL.

These posts:

```
content/writing/example.md
content/notes/example.md
```

both infer the slug:

```
example
```

With:

```toml
url_pattern = "/writing/{slug}/"
```

both derive:

```
https://example.com/writing/example/
```

If directory structure affects your site's public routes, use one of the following approaches:

- give each post a globally unique explicit `slug`
- provide an explicit `canonical_url`
- use a dedicated source adapter that understands the site’s routing model
- choose a generic URL pattern that accurately reflects the site's actual slug-based routes

Always inspect the result with `elsewhere plan`.

## Draft status

Set:

```toml
draft = true
```

to mark the canonical post as a draft.

The default is `false`.

Draft status does not prevent Elsewhere from planning or rendering the post. It produces a warning so that an unpublished source is not mistaken for an already-public canonical page.

Before publishing a derived draft, confirm that the original post is available at its canonical URL.

## Complete example post

The following generic Markdown post exercises the main integration points:

```md
+++
title = "A Tiny Generic Markdown Post"
description = "A short demonstration post for Elsewhere."
date = 2026-07-29
draft = false
slug = "tiny-generic-example"
tags = ["example", "markdown", "posse"]

[elsewhere]
excerpt = "A deliberately small generic Markdown post used to prepare publishing drafts."

[elsewhere.mastodon]
template = """
A tiny example appears.

{excerpt}

{url}
"""

[elsewhere.bluesky]
template = """
New from my website:

{title}

{url}
"""

[elsewhere.reddit]
subreddit = "example"
kind = "link"
title = "{title}"
comment = """
This is the suggested first comment for the example Reddit draft.

{excerpt}

Source:
{url}
"""

[elsewhere.markdown]
template = """
# {title}

_{description}_

{body}

Originally published at {url}
"""
+++

This is a tiny generic Markdown post.

It exists so Elsewhere has something safe, boring, and copy-pastable to render.
```

With:

```toml
site_url = "https://example.com"

[generic]
url_pattern = "/writing/{slug}/"
```

Elsewhere derives:

```
https://example.com/writing/tiny-generic-example/
```

## Complete example configuration

A complete generic configuration might look like this:

```toml
site_url = "https://example.com"
content_dir = "content"
source = "generic"

[defaults]
canonical_phrase = "Originally published on my website:"

[generic]
url_pattern = "/writing/{slug}/"

[mastodon]
max_chars = 500
template = """
{excerpt}

New post: {title}

{url}
"""

[bluesky]
max_chars = 300
template = """
New essay: {title}

{excerpt}

{url}
"""

[reddit]
kind = "link"
subreddit = "example"
title_template = "{title}"
body_template = """
{excerpt}

{url}
"""
comment_template = """
{excerpt}

Originally published here:
{url}
"""
title_max_chars = 300
body_max_chars = 40000
comment_max_chars = 10000

[markdown]
template = """
# {title}

_{description}_

{body}

{canonical_phrase}
{url}"""
```

Renderer tables can be omitted when the built-in defaults are sufficient.

## Plan the post

Run:

```sh
elsewhere plan content/writing/example-post.md
```

Review:

- the title
- the inferred or explicit slug
- the canonical URL
- tags
- draft status
- the selected excerpt
- character counts
- Reddit submission fields
- renderer warnings or errors

The canonical URL deserves particular attention because the generic adapter cannot compare its result with a static-site generator's own routing logic.

## Render drafts

Render a Mastodon draft:

```sh
elsewhere render mastodon content/writing/example-post.md
```

Render a Bluesky draft:

```sh
elsewhere render bluesky content/writing/example-post.md
```

Prepare a Reddit draft:

```sh
elsewhere render reddit content/writing/example-post.md
```

Render the long-form Markdown draft:

```sh
elsewhere render markdown content/writing/example-post.md
```

Save it to a file:

```sh
elsewhere render markdown content/writing/example-post.md > example-post.md.out
```

Render every target:

```sh
elsewhere render all content/writing/example-post.md
```

Elsewhere writes drafts to standard output. It does not publish them.

## Generic does not mean every Markdown format

The generic source is deliberately small.

It does not currently understand:

- YAML front matter
- JSON front matter
- front-matter-free Markdown
- generator-specific configuration
- dated permalink placeholders
- directory-based routes
- page bundles
- collection names
- aliases
- computed permalinks
- template languages
- generated site metadata

It understands Markdown with TOML front matter and a slug-based URL pattern.

For a site that uses more complicated routing, provide an explicit `canonical_url` or add a dedicated source adapter.

## Raw Markdown behaviour

Elsewhere does not run the Markdown through your static-site generator.

The canonical body may still contain source-specific syntax such as:

- shortcodes
- internal links
- template directives
- custom Markdown extensions
- wiki links
- asset paths
- generator-specific markup

A renderer using:

```
{body}
```

receives that syntax unchanged.

The same is true when `{excerpt}` falls back to the first body block.

Use an explicit excerpt or renderer-specific override when the source syntax would not make sense at the destination.

## Unknown fields

Elsewhere ignores front-matter fields outside its canonical model.

For example:

```toml
layout = "post"
authors = ["Example Author"]
category = "notes"
permalink = "/custom/example/"
```

may be meaningful to your site generator, but Elsewhere does not currently expose them to templates or use them for URL construction.

In particular, a generator-specific `permalink` field does not replace Elsewhere's canonical URL.

Use:

```toml
canonical_url = "https://example.com/custom/example/"
```

when the actual route differs from the configured generic pattern.

## Troubleshooting

### Elsewhere says `site_url` is not configured

Generic Markdown requires:

```toml
site_url = "https://example.com"
```

at the top level of `elsewhere.toml`.

Elsewhere does not infer this value from another configuration file.

### The canonical URL is wrong

Check:

1. `canonical_url` in the post
2. the explicit or filename-derived `slug`
3. `generic.url_pattern`
4. `site_url`

Then run:

```sh
elsewhere plan path/to/post.md
```

again.

### A nested post has the wrong URL

The generic source does not use the directory hierarchy.

Provide an explicit slug or canonical URL:

```toml
canonical_url = "https://example.com/notes/software/example-post/"
```

### The `path` field has no effect

The generic adapter does not use `path`.

Use a complete `canonical_url` instead.

### A placeholder remains in the URL

Only `{slug}` is supported in `generic.url_pattern`.

A value such as:

```toml
url_pattern = "/{year}/{slug}/"
```

leaves `{year}` untouched.

Use a slug-only pattern or an explicit canonical URL.

### Elsewhere rejects the front matter

Make sure:

- the first line is `+++`
- the closing delimiter is present
- the contents are valid TOML
- `title` is a string
- recognized optional fields have the expected types

YAML front matter is not supported.

### Raw site syntax appears in a draft

Elsewhere preserves the source Markdown.

Add a clean editorial excerpt:

```toml
[elsewhere]
excerpt = "A destination-safe introduction."
```

or replace the affected renderer template for that post.

### A post-level override has no effect

For generic Markdown, place it beneath:

```toml
[elsewhere]
```

Text renderer overrides use:

```toml
[elsewhere.mastodon]
template = "..."
```

Reddit post-level overrides use:

```text
title
body
comment
```

rather than the site-level `*_template` names.

Also check whether the post contains `[extra.elsewhere]`. When present, it takes precedence over the direct `[elsewhere]` table.

### The post is marked as a draft

Elsewhere reports the source `draft` value as a warning.

It still prepares the output, but the canonical post should normally be public before its derivatives are published.
