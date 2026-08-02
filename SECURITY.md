# Security Policy

Elsewhere takes security vulnerabilities seriously.

If you believe you have found a security vulnerability in Elsewhere, please report it privately through GitHub Security Advisories:

https://github.com/EthanPlant/Elsewhere/security/advisories/new

or through email at [plant.ethan@gmail.com](mailto:plant.ethan@gmail.com).

Do not report security vulnerabilities through public GitHub issues, discussions, or pull requests.

## Reporting a vulnerability

A useful report includes:

- a clear description of the vulnerability
- the affected Elsewhere version or commit
- the operating system and relevant environment details
- steps to reproduce the issue
- the security impact
- any proof-of-concept input or code needed to reproduce it
- any known mitigations or conditions required for exploitation

Please avoid including unrelated private data, credentials, or secrets in a report.

If the vulnerability depends on a particular source file or configuration, include the smallest example necessary to reproduce the behaviour.

## Scope

Elsewhere is a local command-line tool.

Security issues may include unexpected behaviour such as:

- arbitrary code execution caused by processing a post or configuration
- unintended modification or deletion of files
- reading files outside the files required for the requested operation
- exposure of credentials or other secrets
- unsafe handling of untrusted Markdown, TOML, or template input
- path handling that crosses an intended filesystem boundary
- another way for Elsewhere to exceed the authority described by its security model

Ordinary rendering mistakes, incorrect editorial output, unsupported platform behaviour, and discrepancies between configured character limits and a platform's current limits are generally bugs rather than security vulnerabilities unless they create a concrete security impact.

For the project's runtime trust boundaries and local data-handling model, see [`docs/security.md`](docs/security.md).

## Coordinated disclosure

Please give the project a reasonable opportunity to investigate and fix a vulnerability before publishing details that could put users at risk.

Security fixes may be developed privately through GitHub Security Advisories before a patch and advisory are published.

Once a fix is available, affected users should update to a release containing the correction.
