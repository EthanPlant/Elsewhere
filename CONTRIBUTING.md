# Contributing

Thanks for your interest in Elsewhere.

Elsewhere is early, small, and deliberately boring. It reads posts from static sites and renders publishing drafts for other places.

The project is not trying to become a social media dashboard, growth tool, hosted service, or automation hose.

## Development setup

Clone the repository:

```sh
git clone https://github.com/EthanPlant/Elsewhere.git
cd Elsewhere
```

Build the project:

```sh
cargo build
```

Run the tests:

```sh
cargo test
```

Run the full local check:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --locked
```

## Running Elsewhere locally

From the repository root:

```sh
cargo run -- --help
```

Run the example project:

```sh
cd examples/zola
cargo run --manifest-path ../../Cargo.toml -- plan content/writing/example-post.md
cargo run --manifest-path ../../Cargo.toml -- render all content/writing/example-post.md
```

Install the local checkout:

```sh
cargo install --path .
```

Then run:

```sh
elsewhere --help
```

## Project layout

```text
src/
  app.rs          command handlers
  cli.rs          command-line interface
  config.rs       site and renderer configuration
  error.rs        shared error types
  frontmatter.rs  TOML front matter parsing
  plan.rs         plan output
  post.rs         canonical post model
  renderers/      target renderers
  sources/        source readers
  target.rs       render target model
  templates.rs    template replacement
  workspace.rs    config/post loading workflow

docs/
  configuration.md
  renderers.md

examples/
  zola/
```

## Good contributions

Good contributions include:

* bug fixes
* tests
* documentation fixes
* clearer error messages
* improvements to the example project
* small renderer improvements
* better generic Markdown support
* better Zola support

Good first issues are usually small, boring, and easy to verify.

## New source support

A source is responsible for reading a post from a static site and producing Elsewhere’s canonical post model.

Current sources:

```text
generic
zola
```

Possible future sources include Hugo and Eleventy.

New source support should preserve the same core model:

```text
static site post
  -> canonical post
  -> renderer output
```

A source should not publish anything. It should read the site and help Elsewhere understand the canonical post.

## New renderer support

A renderer prepares a publishing draft for another place.

Current renderers:

```text
mastodon
bluesky
reddit
markdown
```

A renderer should:

* use the canonical post model
* preserve the canonical URL when appropriate
* support templates when useful
* produce clear warnings
* avoid hiding platform-specific quirks
* keep platform automation separate from rendering

Some renderers are simple text drafts. Some are structured artifacts.

Mastodon and Bluesky are mostly short text drafts. Markdown is a long-form publishing draft. Reddit is a structured draft with separate fields.

The renderer should reflect the shape of the destination instead of pretending every platform is just one text box.

## Human control

Elsewhere keeps the final decision with the human.

It can prepare drafts, show previews, structure platform-specific output, and warn about obvious problems. It should not post on behalf of the user without an explicit future design decision.

The intended workflow is:

```text
plan
review
render
edit
post manually
```

Please keep that philosophy in mind when proposing features.

A feature that makes thoughtful syndication easier is probably in scope.

A feature that turns Elsewhere into a spray-and-pray spam machine is probably not.

## Non-goals

Elsewhere is not currently trying to provide:

* platform login
* OAuth
* analytics
* scheduling
* automatic posting
* engagement tracking
* a hosted dashboard
* growth tooling
* bulk campaign management

Direct posting APIs may happen later, but rendering should remain useful on its own.

## Pull requests

Please keep pull requests focused.

Before opening a PR, run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --locked
```

Include tests for behaviour changes when practical.

Update documentation when changing user-facing behaviour.

## Commit style

There is no elaborate commit convention.

Prefer short, clear commit messages such as:

```text
Add Zola taxonomy tag parsing
Improve Reddit render warnings
Document Markdown renderer
Fix missing title error
```

## License

By contributing to Elsewhere, you agree that your contribution will be licensed under the same licence as the project.

Elsewhere is licensed under the GNU General Public License, version 3 or later.

See [`LICENSE`](LICENSE) for details.
