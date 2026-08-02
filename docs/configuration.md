# Configuration

Elsewhere is configured with an `elsewhere.toml` file.

The configuration tells Elsewhere:

- which source format your site uses
- where your content lives
- how to construct canonical URLs
- which templates to use for each renderer
- which character limits should produce warnings

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

[defaults]
canonical_phrase = "Originally published on my website:"

[generic]
url_pattern = "/writing/{slug}/"
```

See [Getting Started](getting-started.md) for the complete first-run workflow.

## Create a configuration

Run `elsewhere init` from the root of your static site.

Generic Markdown is the default source:

```sh
elsewhere init
```

You can also select the source explicitly:

```sh
elsewhere init --source generic
```

For a Zola site:

```sh
elsewhere init --source zola
```

Elsewhere writes a starter configuration to `elsewhere.toml` in the current directory.

It will not replace an existing file unless you pass `--force`:

```sh
elsewhere init --source zola --force
```

Using `--force` discards the existing `elsewhere.toml`.

## Configuration discovery

When Elsewhere processes a post, it begins in the post's directory and searches upward through its parent directories for `elsewhere.toml`.

For example:

```
my-site/
├── elsewhere.toml
└── content/
    └── writing/
        └── example-post.md
```

The post at `content/writing/example-post.md` uses the configuration at the root of `my-site`.

If more than one `elsewhere.toml` exists in the directory hierarchy, Elsewhere uses the first one it finds while walking upward. This allows a nested directory to provide a more specific configuration.

The directory containing the selected `elsewhere.toml` becomes the site root. Relative paths such as `content_dir` are resolved from that directory.

## Top-level fields

The top level of `elsewhere.toml` describes the site and selects its source adapter.

```toml
site_url = "https://example.com"
content_dir = "content"
source = "generic"
```

### `site_url`

Type: `string`

Default: none

The public base URL of the site:

```toml
site_url = "https://example.com"
```

Generic Markdown sites must configure `site_url`.

Zola sites may omit it. When `site_url` is absent for a Zola site, Elsewhere reads `base_url` from the site's Zola configuration.

Setting `site_url` in `elsewhere.toml` takes precedence over the value read from Zola.

Elsewhere joins the base URL and derived path with a single `/`, so both of these forms are accepted:

```toml
site_url = "https://example.com"
```

```toml
site_url = "https://example.com/"
```

### `content_dir`

Type: `string`

Default: `"content"`

The directory containing the site's posts, relative to the directory containing `elsewhere.toml`:

```toml
content_dir = "content"
```

For a site with a different layout:

```toml
content_dir = "posts"
```

The Zola source adapter uses this directory when deriving a post’s public path from its location in the content tree.

### `source`

Type: `string`

Default: `"generic"`

The source format used to read posts.

Supported values are:

```
generic
zola
```

The source determines how front matter is interpreted and how canonical URLs are derived.

See [Sources](sources.md), [Using Generic Markdown](generic-markdown.md), and [Using Elsewhere with Zola](zola.md).

## Shared defaults

Shared values belong under `[defaults]`.

```toml
[defaults]
canonical_phrase = "Originally published on my website:"
```

### `canonical_phrase`

Type: `string`

Default: `"Originally published on my website:"`

A phrase available to renderer templates as `{canonical_phrase}`.

It is primarily intended for long-form exports:

```toml
[defaults]
canonical_phrase = "First published at:"
```

```toml
[markdown]
template = """
# {title}

{body}

{canonical_phrase}
{url}"""
```

## Generic Markdown

Generic Markdown settings belong under `[generic]`.

```toml
[generic]
url_pattern = "/writing/{slug}/"
```

### `url_pattern`

Type: `string`

Default: `"/writing/{slug}/"`

The path pattern used to construct a canonical URL for a generic Markdown post.

The supported placeholder is:

```
{slug}
```

For example:

```toml
site_url = "https://example.com"

[generic]
url_pattern = "/notes/{slug}/"
```

A post with the slug `hello` receives the canonical URL:

```
https://example.com/notes/hello/
```

Elsewhere must be able to determine a slug before it can use this pattern.

An explicit `canonical_url` in the post takes precedence over `url_pattern`.

See [Using Generic Markdown](generic-markdown.md) for supported front matter and slug behaviour.

## Zola

Zola settings belong under `[zola]`.

```toml
[zola]
section_url_from_path = true
```

### `section_url_from_path`

Type: `boolean`

Default: `true`

When enabled, Elsewhere derives the canonical path from the post's location beneath `content_dir`.

For example:

```
content/writing/example-post.md
```

produces a path like:

```
/writing/example-post/
```

A Zola `slug` replaces the filename-derived final path segment.

A Zola `path` takes precedence over path derivation.

An explicit `canonical_url` takes precedence over every derived URL.

When `section_url_from_path` is `false`, Elsewhere uses the generic `url_pattern` mechanism instead:

```toml
source = "zola"

[generic]
url_pattern = "/articles/{slug}/"

[zola]
section_url_from_path = false
```

This requires the post to provide or otherwise produce a slug.

See [Using Elsewhere with Zola](zola.md) for the complete URL precedence and supported Zola front matter.

## Renderer configuration

Renderer tables are optional.

When a renderer table is absent, Elsewhere uses that renderer's built-in template and limits.

Site-level renderer configuration can be overridden by an individual post. Template precedence is:

1. the post's renderer-specific template;
2. the renderer template in `elsewhere.toml`;
3. the built-in renderer template.

See [Renderers](renderers.md) for output behaviour and per-post overrides.

## Mastodon

Mastodon settings belong under `[mastodon]`.

```toml
[mastodon]
max_chars = 500
template = """
{excerpt}

New post: {title}

{url}
"""
```

### `max_chars`

Type: non-negative integer

Built-in value: `500`

The character-count threshold used by `plan` and `render`.

Elsewhere warns when the rendered draft exceeds this value. It does not truncate the draft or prevent it from being rendered.

### `template`

Type: `string`

The template used to produce the Mastodon draft.

The built-in template is:

```
{excerpt}

New post: {title}

{url}
```

When a `[mastodon]` table is present, configure both `max_chars` and `template`. Missing fields in this table do not inherit the built-in Mastodon values.

## Bluesky

Bluesky settings belong under `[bluesky]`.

```toml
[bluesky]
max_chars = 300
template = """
New essay: {title}

{excerpt}

{url}
"""
```

### `max_chars`

Type: non-negative integer

Built-in value: `300`

The character-count threshold used by `plan` and `render`.

Elsewhere warns when the rendered draft exceeds this value. It does not shorten the draft automatically.

### `template`

Type: `string`

The template used to produce the Bluesky draft.

The built-in template is:

```
New essay: {title}

{excerpt}

{url}
```

When a `[bluesky]` table is present, configure both `max_chars` and `template`. Missing fields in this table do not inherit the built-in Bluesky values.

## Reddit

Reddit settings belong under `[reddit]`.

The Reddit renderer produces a structured publishing draft rather than one undifferentiated block of text.

A link submission might use:

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
title_max_chars = 300
body_max_chars = 40000
comment_max_chars = 10000
```

A self post might use:

```toml
[reddit]
kind = "selfpost"
subreddit = "example"
title_template = "{title}"
body_template = """
{excerpt}

Read the original:
{url}
"""
title_max_chars = 300
body_max_chars = 40000
comment_max_chars = 10000
```

Unlike the Mastodon and Bluesky tables, omitted Reddit fields inherit Reddit-specific defaults.

### `kind`

Type: `string`

Default: `"link"`

The kind of Reddit submission to prepare.

Supported values are:

```
link
selfpost
```

A link submission uses the canonical URL as the submission URL.

A self post renders `body_template` as the submission body.

### `subreddit`

Type: `string`

Default: none

The proposed subreddit for the draft:

```toml
subreddit = "example"
```

The following values are equivalent:

```toml
subreddit = "example"
```

```toml
subreddit = "r/example"
```

```toml
subreddit = "/r/example"
```

Elsewhere normalizes them to `r/example` in human-readable output.

A missing subreddit produces a warning but does not prevent rendering.

Elsewhere does not validate that the subreddit exists or that the proposed submission complies with its rules.

### `title_template`

Type: `string`

Default: `"{title}"`

The template used for the Reddit submission title:

```toml
title_template = "{title}"
```

### `body_template`

Type: `string`

Default: `"{excerpt}\n\n{url}"`

The body used for a `selfpost` submission:

```toml
body_template = """
{excerpt}

{url}
"""
```

This field is not used for link submissions.

### `comment_template`

Type: `string`

Default: none

An optional suggested first comment for a link submission:

```toml
comment_template = """
Additional context:

{excerpt}

{url}
"""
```

Elsewhere includes the rendered comment in the publishing draft. It does not submit the comment.

This field is not used for self posts.

### `title_max_chars`

Type: non-negative integer

Default: `300`

The warning threshold for the rendered Reddit title.

### `body_max_chars`

Type: non-negative integer

Default: `40000`

The warning threshold for a rendered self-post body.

### `comment_max_chars`

Type: non-negative integer
Default: `10000`

The warning threshold for a rendered suggested first comment.

These limits only produce warnings. Elsewhere does not truncate Reddit fields.

## Markdown

Markdown settings belong under `[markdown]`.

```toml
[markdown]
template = """
# {title}

_{description}_

{body}

{canonical_phrase}
{url}"""
```

### `template`

Type: `string`

The template used to produce a long-form Markdown draft.

The built-in template is:

```
# {title}

_{description}_

{body}

{canonical_phrase}
{url}
```

The Markdown renderer does not copy the source file directly. It reads the canonical post and constructs a new publishing draft without the original front matter.

There is no configured character limit for Markdown output.

## Template variables

Elsewhere templates use variables enclosed in braces:

```toml
template = """
New post: {title}

{excerpt}

{url}
"""
```

Elsewhere supports the following variables.

| Variable             | Value                                   |
| -------------------- | --------------------------------------- |
| `{title}`            | Post title                              |
| `{description}`      | Post description                        |
| `{excerpt}`          | Editorial excerpt selected by Elsewhere |
| `{first_paragraph}`  | First paragraph of the post body        |
| `{url}`              | Canonical URL                           |
| `{canonical_url}`    | Alias for `{url}`                       |
| `{date}`             | Post date                               |
| `{slug}`             | Post slug                               |
| `{tags}`             | Tags joined with `, `                   |
| `{body}`             | Markdown body without front matter      |
| `{body_markdown}`    | Alias for `{body}`                      |
| `{canonical_phrase}` | Shared phrase from `[defaults]`         |

The variable name may contain surrounding whitespace:

```
{ title }
```

is treated as:

```
{title}
```

Templates do not support expressions, filters, conditionals, or arbitrary code.

Unknown variables are errors:

```
{author}
```

will fail because `author` is not part of the current canonical post model.

An opening `{` without a closing `}` is also an error.

## Required template values

Some variables require the source post to contain a corresponding value.

These variables fail when their value is unavailable:

- `{description}`
- `{first_paragraph}`
- `{url}` and `{canonical_url}`
- `{date}`
- `{slug}`

The following variables always produce a value:

- `{title}`
- `{excerpt}`
- `{tags}`
- `{body}` and `{body_markdown}`
- `{canonical_phrase}`

An empty tag list renders `{tags}` as an empty string.

Use `elsewhere plan` after changing a template. Renderer-specific template failures are reported in the plan.

## Editorial excerpts

The `{excerpt}` variable uses the first available value in this order:

1. the post's explicit Elsewhere excerpt
2. the post description
3. the first paragraph of the body
4. the post title

This makes `{excerpt}` safe to use in a site-wide template even when individual posts do not provide a custom excerpt.

Source-specific documentation explains how to set an explicit excerpt:

* [Using Elsewhere with Zola](zola.md);
* [Using Generic Markdown](generic-markdown.md).

## Per-post overrides

The site configuration defines defaults for every post. Individual posts can override editorial and renderer-specific fields.

Zola posts place Elsewhere metadata under:

```toml
[extra.elsewhere]
```

Generic Markdown posts place it under:

```toml
[elsewhere]
```

For example, a Zola post can override its Mastodon template:

```toml
[extra.elsewhere.mastodon]
template = """
A different introduction for this post.

{excerpt}

{url}
"""
```

The equivalent generic Markdown post uses:

```toml
[elsewhere.mastodon]
template = """
A different introduction for this post.

{excerpt}

{url}
"""
```

Supported renderer override sections are:

```
mastodon
bluesky
reddit
markdown
```

Mastodon, Bluesky, and Markdown overrides replace the template for that post.

Reddit overrides are merged field by field with the site-level Reddit configuration. A post can change its subreddit, kind, title template, body template, or comment template while retaining other site defaults.

Character limits are configured at the site level and cannot currently be overridden per post.

See [Renderers](renderers.md) for the complete override model.

## Complete example

The following configuration enables every current renderer for a Zola site:

```toml
content_dir = "content"
source = "zola"

[defaults]
canonical_phrase = "Originally published on my website:"

[zola]
section_url_from_path = true

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

Review the effective output with:

```sh
elsewhere plan content/writing/example-post.md
```

Configuration changes do not publish anything. They only change the drafts Elsewhere prepares.
