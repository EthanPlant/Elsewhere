# JSON Schemas

Elsewhere can emit a machine-readable publishing plan as JSON.

```sh
elsewhere plan --json content/writing/example-post.md
```

The JSON output describes:

- the canonical post as Elsewhere understands it
- the proposed result for every renderer
- renderer statuses
- character counts and configured limits
- rendered previews
- warnings and errors
- structured Reddit submission fields

Planning does not publish anything.

At this revision, Elsewhere does not ship a formal JSON Schema file and the output does not contain a `schema_version` field. Treat the structure documented here as specific to the installed Elsewhere version.

## Writing a plan to a file

Redirect the JSON output to a file:

```sh
elsewhere plan --json content/writing/example-post.md > plan.json
```

Inspect it with `jq`:

```sh
jq . plan.json
```

Elsewhere writes pretty-printed UTF-8 JSON to standard output, followed by a newline.

Do not depend on indentation, whitespace, or object key ordering.

## Top-level structure

A plan has this shape:

```json
{
  "canonical": {
    "title": "A Tiny Example Post",
    "canonical_url": "https://example.com/writing/example-post/",
    "tags": [
      "example",
      "posse"
    ],
    "draft": false
  },
  "targets": [
    {
      "target": "mastodon",
      "status": "ready",
      "length": 66,
      "max_length": 500,
      "preview": "A tiny example appears.\n\nhttps://example.com/writing/example-post/"
    },
    {
      "target": "bluesky",
      "status": "ready",
      "length": 72,
      "max_length": 300,
      "preview": "New post: A Tiny Example Post\n\nhttps://example.com/writing/example-post/"
    },
    {
      "target": "markdown",
      "status": "ready",
      "length": 119,
      "output": "use `elsewhere render markdown content/writing/example-post.md > markdown.md`"
    },
    {
      "target": "reddit",
      "status": "ready",
      "length": 243,
      "preview": "Subreddit: r/example\nKind: link\n\nTitle:\nA Tiny Example Post\n\nURL:\nhttps://example.com/writing/example-post/\n\nSuggested first comment:\nOriginal post: https://example.com/writing/example-post/\n\nReminder: check the subreddit rules before posting.",
      "artifact": {
        "type": "reddit",
        "subreddit": "example",
        "kind": "link",
        "title": "A Tiny Example Post",
        "url": "https://example.com/writing/example-post/",
        "comment": "Original post: https://example.com/writing/example-post/"
      }
    }
  ]
}
```

This example shows the available field shapes. The actual text, lengths, limits, warnings, and output instructions depend on the source post and configuration.

## `PlanOutput`

The root object represents one complete plan.

| Field       | Type             | Required | Description                                                        |
| ----------- | ---------------- | -------: | ------------------------------------------------------------------ |
| `canonical` | object           |      Yes | Source-neutral information about the original post                 |
| `targets`   | array            |      Yes | One plan for each supported renderer                               |
| `warnings`  | array of strings |       No | Warnings that apply to the canonical post rather than one renderer |

In TypeScript-like notation:

```ts
interface PlanOutput {
  canonical: PlanCanonical;
  targets: PlanTarget[];
  warnings?: string[];
}
```

### `warnings`

The top-level `warnings` field is omitted when there are no canonical warnings.

At this revision, a draft post produces:

```json
{
  "warnings": [
    "post is marked as draft"
  ]
}
```

The draft warning appears at the top level rather than being repeated in every target.

Consumers should treat warning strings as human-readable explanations. Do not use the exact wording as a stable machine identifier.

## `PlanCanonical`

The `canonical` object describes the shared post passed to every renderer.

```ts
interface PlanCanonical {
  title: string;
  canonical_url?: string;
  tags: string[];
  draft: boolean;
}
```

| Field           | Type             | Required | Description                                  |
| --------------- | ---------------- | -------: | -------------------------------------------- |
| `title`         | string           |      Yes | Canonical post title                         |
| `canonical_url` | string           |       No | Public URL of the original post              |
| `tags`          | array of strings |      Yes | Canonical tags, possibly empty               |
| `draft`         | boolean          |      Yes | Whether the source post is marked as a draft |

### `title`

The canonical title:

```json
{
  "title": "A Tiny Example Post"
}
```

This field is always present for a successfully loaded post.

### `canonical_url`

The public URL of the original post:

```json
{
  "canonical_url": "https://example.com/writing/example-post/"
}
```

When Elsewhere cannot determine a canonical URL, the field is omitted.

It is not emitted as `null`.

Consumers should therefore test for the field's presence:

```sh
jq -e '.canonical.canonical_url != null' plan.json
```

### `tags`

The canonical tag list:

```json
{
  "tags": [
    "indieweb",
    "posse"
  ]
}
```

When the post has no tags, Elsewhere emits an empty array:

```json
{
  "tags": []
}
```

The field is not omitted.

### `draft`

The canonical draft state:

```json
{
  "draft": false
}
```

When `draft` is `true`, the plan also contains a top-level warning.

## `PlanTarget`

Each entry in `targets` describes one renderer.

```ts
interface PlanTarget {
  target: RenderTarget;
  status: PlanStatus;
  length?: number;
  max_length?: number;
  preview?: string;
  output?: string;
  warnings?: string[];
  error?: string;
  artifact?: RenderedArtifact;
}
```

| Field        | Type             | Required | Description                                                     |
| ------------ | ---------------- | -------: | --------------------------------------------------------------- |
| `target`     | string enum      |      Yes | Renderer name                                                   |
| `status`     | string enum      |      Yes | `ready`, `warning`, or `error`                                  |
| `length`     | integer          |       No | Character count of the complete rendered output                 |
| `max_length` | integer          |       No | Configured limit for the complete rendered output               |
| `preview`    | string           |       No | Complete rendered preview for a short-form or structured target |
| `output`     | string           |       No | Human-readable rendering instruction for a long-form target     |
| `warnings`   | array of strings |       No | Non-fatal renderer warnings                                     |
| `error`      | string           |       No | Renderer error message                                          |
| `artifact`   | object           |       No | Structured renderer-specific output                             |

Fields whose values are unavailable are generally omitted rather than emitted as `null`.

## Renderer names

The `target` field is one of:

```
mastodon
bluesky
markdown
reddit
```

For example:

```json
{
  "target": "mastodon"
}
```

At this revision, the `targets` array is produced in this order:

```
mastodon
bluesky
markdown
reddit
```

Consumers should not depend on array position. Find entries using the `target` field:

```sh
jq '.targets[] | select(.target == "reddit")' plan.json
```

Future versions may add renderers or change their ordering.

## Status values

The `status` field is one of:

```
ready
warning
error
```

### `ready`

The renderer successfully prepared its output and produced no renderer-specific warnings.

```json
{
  "target": "mastodon",
  "status": "ready",
  "length": 145,
  "max_length": 500,
  "preview": "..."
}
```

`ready` means that Elsewhere could prepare the draft. It does not mean that the draft has been reviewed or approved for publication.

### `warning`

The renderer prepared output, but found one or more conditions requiring attention.

```json
{
  "target": "mastodon",
  "status": "warning",
  "length": 512,
  "max_length": 500,
  "preview": "...",
  "warnings": [
    "mastodon render is 512 characters. Configured limit is 500."
  ]
}
```

A warning does not prevent rendering.

### `error`

The renderer could not prepare output:

```json
{
  "target": "mastodon",
  "status": "error",
  "error": "template references unavailable value: description"
}
```

An error target omits:

* `length`;
* `max_length`;
* `preview`;
* `output`;
* `warnings`; and
* `artifact`.

Error strings are intended for people. Consumers should use `status == "error"` rather than matching exact error text.

## `length`

`length` is the character count of the renderer's complete textual output.

```json
{
  "length": 145
}
```

Elsewhere counts Rust `char` values. This is a count of Unicode scalar values, not:

- UTF-8 bytes
- user-perceived grapheme clusters
- words
- destination-specific weighted characters
- the final count calculated by a platform

For ordinary Latin text, this generally corresponds to the expected character count. More complex Unicode sequences may be counted differently by a destination.

### Reddit length

For Reddit, `length` counts the complete human-readable publishing draft in `preview`, including headings and the final reminder.

It is not the length of only the Reddit title, body, or comment.

Reddit field limits are evaluated separately, but their individual numeric counts are not currently exposed as structured JSON fields. When one exceeds its limit, Elsewhere emits a warning such as:

```json
{
  "warnings": [
    "reddit title is 315 characters. Configured limit is 300."
  ]
}
```

## `max_length`

`max_length` is the configured limit for the complete rendered output:

```json
{
  "max_length": 500
}
```

It is currently present for:

- Mastodon
- Bluesky.

It is omitted for:

- Markdown
- Reddit

Reddit uses separate title, body, and comment limits rather than one limit for the human-readable draft.

Exceeding `max_length` changes the target status to `warning`. Elsewhere does not truncate the output.

## `preview`

`preview` contains the renderer's complete textual output:

```json
{
  "preview": "A tiny example appears.\n\nhttps://example.com/writing/example-post/"
}
```

It is currently present for:

- Mastodon
- Bluesky
- Reddit

It is omitted for Markdown because Markdown is treated as long-form output.

JSON newline escapes represent actual newline characters in the rendered string.

### Sensitive content

A preview may contain:

- unpublished titles
- excerpts
- canonical URLs
- Reddit submission details
- comments
- other post content

Treat plan files and CI logs as unpublished editorial material when the source post is not yet public.

## `output`

Long-form targets use `output` instead of `preview`.

For Markdown:

```json
{
  "output": "use `elsewhere render markdown content/writing/example-post.md > markdown.md`"
}
```

This is a human-readable instruction.

It is not:

- a generated filename
- a guarantee that a file exists
- a shell-safe structured command
- a stable path field
- the rendered Markdown itself

Consumers should not parse `output` to construct commands. Invoke Elsewhere directly instead:

```sh
elsewhere render markdown content/writing/example-post.md > markdown.md
```

## Target warnings

The target-level `warnings` field is omitted when it would be empty.

Example:

```json
{
  "target": "reddit",
  "status": "warning",
  "length": 156,
  "preview": "...",
  "warnings": [
    "reddit subreddit is not configured. Check the destination community before posting."
  ],
  "artifact": {
    "type": "reddit",
    "subreddit": null,
    "kind": "link",
    "title": "A Tiny Example Post",
    "url": "https://example.com/writing/example-post/"
  }
}
```

Renderer warnings do not include the `Warning: ` prefix used by ordinary rendered diagnostics.

The JSON plan normalizes:

```
Warning: reddit subreddit is not configured.
```

into a plain warning string.

As with errors, warning text is human-readable and should not be treated as a stable identifier.

## `artifact`

`artifact` contains structured output that cannot be represented adequately by one text preview.

At this revision, the only artifact type is Reddit.

```ts
type RenderedArtifact =
  | RedditLinkArtifact
  | RedditSelfPostArtifact;
```

Successful Mastodon, Bluesky, and Markdown targets omit `artifact`.

A failed Reddit target also omits it.

## Reddit artifacts

Every successful Reddit plan includes:

```json
{
  "artifact": {
    "type": "reddit",
    "subreddit": "example",
    "kind": "link",
    "title": "A Tiny Example Post",
    "url": "https://example.com/writing/example-post/"
  }
}
```

The `type` discriminator is always:

```
reddit
```

The `kind` field distinguishes link submissions from self posts.

## Reddit link artifact

A link artifact has this shape:

```ts
interface RedditLinkArtifact {
  type: "reddit";
  subreddit: string | null;
  kind: "link";
  title: string;
  url: string;
  comment?: string;
}
```

Example:

```json
{
  "type": "reddit",
  "subreddit": "indieweb",
  "kind": "link",
  "title": "Your Website Should Be the Source of Truth",
  "url": "https://example.com/writing/source-of-truth/",
  "comment": "I wrote some additional context here."
}
```

| Field       | Type             | Required | Description                            |
| ----------- | ---------------- | -------: | -------------------------------------- |
| `type`      | `"reddit"`       |      Yes | Artifact discriminator                 |
| `subreddit` | string or `null` |      Yes | Normalized subreddit name without `r/` |
| `kind`      | `"link"`         |      Yes | Reddit submission kind                 |
| `title`     | string           |      Yes | Rendered submission title              |
| `url`       | string           |      Yes | Canonical submission URL               |
| `comment`   | string           |       No | Rendered suggested first comment       |

### `subreddit`

Configured forms such as:

```
example
r/example
/r/example
```

are normalized to:

```json
{
  "subreddit": "example"
}
```

Unlike most optional plan fields, `subreddit` is emitted as `null` when it is not configured:

```json
{
  "subreddit": null
}
```

A missing subreddit also produces a target warning.

### `url`

A link artifact includes `url`.

When a canonical URL is unavailable, the current renderer may emit an empty string:

```json
{
  "url": ""
}
```

Consumers preparing a link submission should therefore validate that the value is non-empty.

### `comment`

`comment` is included only when a comment template was configured and successfully rendered.

Without one, the field is omitted.

A link artifact never includes `body`.

## Reddit self-post artifact

A self-post artifact has this shape:

```ts
interface RedditSelfPostArtifact {
  type: "reddit";
  subreddit: string | null;
  kind: "selfpost";
  title: string;
  body: string;
}
```

Example:

```json
{
  "type": "reddit",
  "subreddit": "indieweb",
  "kind": "selfpost",
  "title": "Your Website Should Be the Source of Truth",
  "body": "The original belongs on your website.\n\nhttps://example.com/writing/source-of-truth/"
}
```

| Field       | Type             | Required | Description               |
| ----------- | ---------------- | -------: | ------------------------- |
| `type`      | `"reddit"`       |      Yes | Artifact discriminator    |
| `subreddit` | string or `null` |      Yes | Normalized subreddit name |
| `kind`      | `"selfpost"`     |      Yes | Reddit submission kind    |
| `title`     | string           |      Yes | Rendered submission title |
| `body`      | string           |      Yes | Rendered self-post body   |

A self-post artifact omits:

- `url`
- `comment`

The canonical URL may still appear inside `body` when its template uses `{url}`.

## Optional fields are omitted

Most optional fields use omission rather than explicit `null`.

For example, a successful Markdown target is:

```json
{
  "target": "markdown",
  "status": "ready",
  "length": 390,
  "output": "use `elsewhere render markdown content/writing/example-post.md > markdown.md`"
}
```

It is not:

```json
{
  "target": "markdown",
  "status": "ready",
  "length": 390,
  "max_length": null,
  "preview": null,
  "output": "use `elsewhere render markdown content/writing/example-post.md > markdown.md`",
  "warnings": [],
  "error": null,
  "artifact": null
}
```

Consumers should tolerate absent keys.

A useful pattern in `jq` is:

```sh
jq '.warnings // []' plan.json
```

The principal exception is `artifact.subreddit`, which is emitted as either a string or `null`.

## Complete structural definition

The current output can be summarized as:

```ts
type RenderTarget =
  | "mastodon"
  | "bluesky"
  | "markdown"
  | "reddit";

type PlanStatus =
  | "ready"
  | "warning"
  | "error";

interface PlanOutput {
  canonical: {
    title: string;
    canonical_url?: string;
    tags: string[];
    draft: boolean;
  };

  targets: Array<{
    target: RenderTarget;
    status: PlanStatus;
    length?: number;
    max_length?: number;
    preview?: string;
    output?: string;
    warnings?: string[];
    error?: string;
    artifact?: RedditArtifact;
  }>;

  warnings?: string[];
}

type RedditArtifact =
  | {
      type: "reddit";
      subreddit: string | null;
      kind: "link";
      title: string;
      url: string;
      comment?: string;
    }
  | {
      type: "reddit";
      subreddit: string | null;
      kind: "selfpost";
      title: string;
      body: string;
    };
```

This is descriptive notation, not a generated TypeScript declaration or formal JSON Schema.

## Command errors and target errors

There are two distinct kinds of failure.

### The plan cannot be constructed

Elsewhere returns a command error when it cannot load the post or configuration.

Examples include:

- the post does not exist
- `elsewhere.toml` cannot be found
- front matter is invalid
- a required canonical field is missing
- the source configuration cannot be loaded
- the canonical post cannot be constructed

In this case, Elsewhere does not emit a complete plan object.

A script should first check whether the command itself succeeded:

```sh
if ! elsewhere plan --json content/writing/example-post.md > plan.json; then
  echo "Elsewhere could not construct a plan" >&2
  exit 1
fi
```

### One renderer fails

Renderer failures are represented inside a completed plan:

```json
{
  "target": "mastodon",
  "status": "error",
  "error": "template contains unknown variable: author"
}
```

At this revision, the presence of a target with `status: "error"` does not itself cause `elsewhere plan --json` to return a non-zero exit code.

Automation must inspect the target statuses explicitly.

## Checking a plan in CI

To fail when any renderer has an error:

```sh
elsewhere plan --json content/writing/example-post.md > plan.json

jq -e '
  all(.targets[]; .status != "error")
' plan.json > /dev/null
```

To require every target to be completely ready:

```sh
elsewhere plan --json content/writing/example-post.md > plan.json

jq -e '
  ((.warnings // []) | length == 0) and
  all(.targets[]; .status == "ready")
' plan.json > /dev/null
```

This stricter check fails on:

- canonical warnings
- character-limit warnings
- missing Reddit community warnings
- renderer errors

Whether warnings should fail CI is a project decision.

## Finding a target

Do not rely on array indexes.

Select a target by name:

```sh
jq '.targets[] | select(.target == "mastodon")' plan.json
```

Get its status:

```sh
jq -r '
  .targets[]
  | select(.target == "mastodon")
  | .status
' plan.json
```

Get the Reddit artifact:

```sh
jq '
  .targets[]
  | select(.target == "reddit")
  | .artifact
' plan.json
```

Extract a Reddit title:

```sh
jq -r '
  .targets[]
  | select(.target == "reddit")
  | .artifact.title
' plan.json
```

Check for a canonical URL:

```sh
jq -e '
  (.canonical.canonical_url // "") != ""
' plan.json > /dev/null
```

## Compatibility

At this revision:

- there is no `schema_version`
- there is no published `.schema.json` file
- warning and error messages are not stable identifiers
- `output` is human-readable guidance
- target array ordering should not be treated as a contract
- pretty-print formatting should not be treated as a contract

Tools consuming plans should:

1. pin the Elsewhere version used in automation
2. identify targets by `target`
3. identify artifacts by `type` and `kind`
4. tolerate unknown object fields
5. tolerate omitted optional fields
6. avoid matching full warning or error strings
7. validate required values such as canonical and Reddit URLs
8. review Elsewhere release notes before upgrading

A future schema version may formalize stronger compatibility guarantees. Until then, the installed Elsewhere release is the authoritative definition of its JSON output.

## Plans are review artifacts

The JSON plan is intended to make Elsewhere's proposed work inspectable.

It may contain complete social drafts, Reddit bodies, suggested comments, unpublished URLs, and other editorial material. It is not merely harmless build metadata.

Do not expose plan files through:

- public CI artifacts
- public build logs
- repository commits
- shared caches
- debugging output

unless the underlying post content is already public and you intend the plan to be public as well.
