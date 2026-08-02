# Using Elsewhere with Zola

Elsewhere can read posts directly from a Zola site.

The Zola integration uses:

- the site's `zola.toml` to find its public `base_url`
- the post's location under `content/` to derive its canonical URL
- ordinary Zola page front matter for titles, descriptions, dates, tags, slugs, paths, and draft status
- `[extra.elsewhere]` for syndication-specific editorial choices

Elsewhere does not run Zola or render your site. It reads the source Markdown and turns it into publishing drafts.

## Initialize Elsewhere

Run the following command from the root of your Zola site:

```sh
elsewhere init --source zola
```

This creates an `elsewhere.toml` file in the current directory.

A minimal Zola configuration looks like this:

```toml
content_dir = "content"
source = "zola"

[defaults]
canonical_phrase = "Originally published on my website:"

[zola]
section_url_from_path = true
```

The generated configuration may also include renderer tables for Mastodon, Bluesky, Reddit, and Markdown.

See [Configuration](configuration.md) for the complete `elsewhere.toml` reference.

## Project layout

A typical Zola site using Elsewhere looks like this:

```
my-site/
├── elsewhere.toml
├── zola.toml
└── content/
    └── writing/
        └── example-post.md
```

The directory containing `elsewhere.toml` becomes the site root.

Relative paths such as `content_dir` and the post path are resolved from that directory.

Elsewhere searches for `elsewhere.toml` starting from the post's directory and walking upward. It does not require the command to be run from precisely one directory, provided the post path can be resolved and a configuration exists above it.

## Zola site configuration

Elsewhere normally reads the public site URL from `base_url` in `zola.toml`:

```toml
base_url = "https://example.com"
```

A minimal `zola.toml` is enough:

```toml
base_url = "https://example.com"
title = "Example Zola Site"
```

The configured `base_url` is joined with the path derived for each post.

### Override the site URL

You can set `site_url` directly in `elsewhere.toml`:

```toml
site_url = "https://example.com"
content_dir = "content"
source = "zola"
```

When `site_url` is present, Elsewhere uses it instead of reading `base_url` from `zola.toml`.

This can be useful when:

- you are preparing drafts against a different public origin
- the site configuration is not available in the same checkout
- you want Elsewhere's URL source to be explicit

Use the actual public origin of the canonical site. A staging or local development URL will be copied into every renderer that uses `{url}`.

## A minimal Zola post

Elsewhere expects TOML front matter delimited by `+++`.

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

Render every target with:

```sh
elsewhere render all content/writing/example-post.md
```

## Supported front matter

Elsewhere reads a deliberately small subset of the post's front matter into its canonical post model.

| Field             | Required | Description                           |
| ----------------- | -------: | ------------------------------------- |
| `title`           |      Yes | Post title                            |
| `description`     |       No | Short description or summary          |
| `date`            |       No | Publication date or datetime          |
| `taxonomies.tags` |       No | Zola tags                             |
| `tags`            |       No | Fallback top-level tag array          |
| `slug`            |       No | Explicit final URL segment            |
| `path`            |       No | Explicit public path                  |
| `canonical_url`   |       No | Complete canonical URL                |
| `draft`           |       No | Whether the page is a draft           |
| `extra.elsewhere` |       No | Elsewhere-specific editorial metadata |

Other Zola fields remain available to Zola but are not currently exposed to Elsewhere's templates.

## Draft status

Elsewhere reads Zola's `draft` field:

```toml
draft = true
```

The default is `false`.

A draft post can still be planned and rendered. Elsewhere reports a warning rather than blocking the operation.

This lets you prepare publishing drafts before the source post goes live while making the unpublished state visible during review.

Before publishing a derived draft, confirm that the canonical post is available on your website.

## Canonical URL derivation

Elsewhere attempts to determine the public URL of the original post.

For a Zola source, URL selection follows this order:

1. `canonical_url` in the post
2. `path` in the post
3. the post's location beneath `content_dir`, when `section_url_from_path = true`
4. the generic slug pattern, when `section_url_from_path = false`

Review the result with:

```sh
elsewhere plan content/writing/example-post.md
```

The canonical URL should point to the original page on your website.

## Explicit canonical URL

An explicit `canonical_url` takes precedence over every derived value:

```toml
canonical_url = "https://example.com/essays/a-special-address/"
```

Elsewhere uses this value as written.

This is useful when the public URL cannot be inferred from the source path or when the site uses routing rules Elsewhere does not understand.

## Explicit path

Zola's `path` field provides an explicit public path:

```toml
path = "/essays/a-special-address/"
```

Given:

```toml
base_url = "https://example.com"
```

Elsewhere derives:

```
https://example.com/essays/a-special-address/
```

An explicit `canonical_url` still takes precedence over `path`.

## URLs from the content path

The default Zola configuration uses:

```toml
[zola]
section_url_from_path = true
```

Elsewhere then derives the public path from the post's location beneath `content_dir`.

For example:

```
content/writing/example-post.md
```

becomes:

```
/writing/example-post/
```

With a site URL of `https://example.com`, the canonical URL is:

```
https://example.com/writing/example-post/
```

Nested directories remain part of the path:

```
content/notes/software/example-post.md
```

becomes:

```
https://example.com/notes/software/example-post/
```

The `.md` extension is removed and a trailing slash is added.

## Slugs

An explicit Zola `slug` replaces the filename-derived final segment:

```toml
slug = "a-better-address"
```

For:

```
content/writing/original-filename.md
```

Elsewhere derives:

```
https://example.com/writing/a-better-address/
```

The parent directories still come from the post's location beneath `content_dir`.

The slug is also available to renderer templates as:

```
{slug}
```

## Disable path-based section URLs

You can disable content-path derivation:

```toml
[zola]
section_url_from_path = false
```

Elsewhere then applies the generic URL pattern to the post's slug.

For example:

```toml
source = "zola"

[generic]
url_pattern = "/articles/{slug}/"

[zola]
section_url_from_path = false
```

A post with:

```toml
slug = "example-post"
```

receives:

```text
https://example.com/articles/example-post/
```

When no `[generic]` table is configured, Elsewhere uses the default pattern:

```
/writing/{slug}/
```

Disabling path-based URLs requires Elsewhere to determine a slug from the post or its filename.

## Page bundles and index files

Elsewhere's path derivation is intentionally simple.

It does not currently special-case Zola page bundles or section files.

For example:

```
content/writing/example-post/index.md
```

is mechanically derived as:

```
/writing/example-post/index/
```

Elsewhere does not infer that Zola may publish the bundle at:

```
/writing/example-post/
```

Similarly, `_index.md` is not given special section behaviour.

For posts whose Zola URL does not correspond directly to their Markdown path, provide an explicit `path` or `canonical_url`:

```toml
path = "/writing/example-post/"
```

Always verify bundle and section URLs with `elsewhere plan`.

## Elsewhere metadata

Zola provides `[extra]` for project-specific metadata.

Elsewhere uses:

```toml
[extra.elsewhere]
```

for editorial information that belongs to syndication rather than the canonical page itself.

A complete post might begin:

```toml
+++
title = "A Tiny Example Post"
description = "A short demonstration post."
date = 2026-07-29

[taxonomies]
tags = ["example", "markdown", "posse"]

[extra.elsewhere]
excerpt = "A deliberately small example used to prepare syndication drafts."
+++
```

The values under `[extra.elsewhere]` do not change how Zola renders the page unless your own templates choose to use them.

## Complete example post

The following post exercises the main Zola integration points:

```md
+++
title = "A Tiny Example Post"
description = "A short demonstration post for Elsewhere."
date = 2026-07-29
draft = false
slug = "tiny-example"

[taxonomies]
tags = ["example", "markdown", "posse"]

[extra.elsewhere]
excerpt = "This is a deliberately small example post used to prepare syndication drafts."

[extra.elsewhere.mastodon]
template = """
A tiny example appears.

{excerpt}

{url}
"""

[extra.elsewhere.bluesky]
template = """
New from my website:

{title}

{url}
"""

[extra.elsewhere.reddit]
subreddit = "example"
kind = "link"
title = "{title}"
comment = """
This is the suggested first comment for the example Reddit draft.

{excerpt}

Source:
{url}
"""

[extra.elsewhere.markdown]
template = """
# {title}

_{description}_

{body}

Originally published at {url}
"""
+++

This is a tiny example post.

It exists so Elsewhere has something safe, boring, and copy-pastable to render.
```

Given the path:

```
content/writing/example-post.md
```

and the explicit slug:

```
tiny-example
```

Elsewhere derives:

```
https://example.com/writing/tiny-example/
```

unless `path` or `canonical_url` overrides it.

## Plan the post

Run:

```sh
elsewhere plan content/writing/example-post.md
```

Review:

- the title
- the canonical URL
- the tags
- draft status
- the selected excerpt
- the effective renderer templates
- character counts
- Reddit submission fields
- warnings or errors

Planning is especially important when using Zola paths that do not correspond directly to their location under `content/`.

## Render drafts

Render a single target:

```sh
elsewhere render mastodon content/writing/example-post.md
```

```sh
elsewhere render bluesky content/writing/example-post.md
```

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

## Raw Markdown behaviour

Elsewhere does not invoke Zola's Markdown renderer.

The canonical body is the Markdown source after the closing front-matter delimiter.

This means the body may still contain Zola-specific syntax such as:

- shortcodes;
- internal `@/` links;
- colocated asset references;
- template-specific conventions; or
- other markup intended to be processed by Zola.

A renderer using:

```
{body}
```

receives that source syntax unchanged.

Short-form templates that use `{excerpt}` may also receive Zola-specific syntax when the excerpt falls back to the first body block.

Provide an explicit `[extra.elsewhere]` excerpt or a renderer override when raw Zola markup would not make sense at the destination.

## Fields Elsewhere does not currently use

Elsewhere does not currently expose arbitrary Zola front matter to templates.

Fields such as the following may still be meaningful to Zola but are not part of Elsewhere's canonical post:

- `updated`;
- `authors`;
- `aliases`;
- `template`;
- `page_template`;
- `weight`;
- taxonomies other than `tags`;
- arbitrary values elsewhere under `extra`; and
- generated Zola permalink data.

Elsewhere also does not ask Zola to calculate the final permalink. It derives the URL from its own configuration and the source post.

Use `path` or `canonical_url` when the final URL depends on Zola behaviour Elsewhere does not model.

## Runnable example

The repository contains a complete example under:

```
examples/zola/
```

Its layout is:

```
examples/zola/
├── elsewhere.toml
├── zola.toml
└── content/
    └── writing/
        └── example-post.md
```

From the Elsewhere repository root:

```sh
cd examples/zola
```

Plan the example:

```sh
cargo run --manifest-path ../../Cargo.toml -- \
  plan content/writing/example-post.md
```

Render every target:

```sh
cargo run --manifest-path ../../Cargo.toml -- \
  render all content/writing/example-post.md
```

Export its Markdown draft:

```sh
cargo run --manifest-path ../../Cargo.toml -- \
  render markdown content/writing/example-post.md > example-post.md.out
```

## Troubleshooting

### Elsewhere cannot read the Zola configuration

At this revision, Elsewhere looks for:

```
zola.toml
```

in the directory containing `elsewhere.toml`.

Make sure the file exists and contains a string `base_url`:

```toml
base_url = "https://example.com"
```

Alternatively, set `site_url` directly in `elsewhere.toml`.

### The canonical URL is wrong

Check, in order:

1. `canonical_url`
2. `path`
3. `slug`
4. the post's location beneath `content_dir`
5. `section_url_from_path`
6. `site_url` or Zola's `base_url`

Then run `elsewhere plan` again.

### A page bundle includes `/index/`

Elsewhere does not special-case `index.md`.

Set the public path explicitly:

```toml
path = "/writing/example-post/"
```

### Zola markup appears in the rendered draft

Elsewhere preserves the raw Markdown body.

Use an explicit excerpt for short-form output:

```toml
[extra.elsewhere]
excerpt = "A clean introduction for syndication."
```

For long-form output, edit the rendered draft or provide a per-post Markdown template that avoids incompatible source material.

### A post-level override has no effect

Make sure it is nested beneath:

```toml
[extra.elsewhere]
```

For text renderers, use:

```toml
[extra.elsewhere.mastodon]
template = "..."
```

For Reddit, use the post-level keys:

```text
title
body
comment
```

rather than the site-level `*_template` names.

Run `elsewhere plan` to inspect the effective output.

### The post is marked as a draft

Elsewhere preserves Zola's `draft` status and reports a warning.

It does not prevent you from preparing output, but you should normally publish the canonical page before publishing its derivatives.
