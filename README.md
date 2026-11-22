# Review Stats

Stats concerning PR reviews on GitHub.

## Quickstart

This is a Rust CLI tool.
You can use Cargo to run: `cargo run -- <args>`.
Run with `--help` for usage information and examples.

### Authentication

Provide a personal access token through the `GITHUB_TOKEN` environment variable to raise GitHub's hourly API limits from
60 to 5K. A fine-grained token with read-only "Pull requests" access is enough.

If you use the [GitHub CLI](https://cli.github.com/), you can reuse its token:

```bash
GITHUB_TOKEN=$(gh auth token) cargo run -- <args>
```

Or export to run it multiple times:

```bash
export GITHUB_TOKEN=$(gh auth token)
```

### Troubleshooting

Run with `--verbose` (or `-v`) to enable debug logging.
An unset or empty `GITHUB_TOKEN` silently falls back to anonymous access; debug logs show an
"Authenticating with personal token" line when the token is picked up.

## Build and test

You can also build and test with Cargo:

- Build: `cargo build`
- Test: `cargo test`

### Unit tests

Unit tests are located in the [tests](./tests) subfolder, to prevent access to private fields.
This is intentional.
The goal is to enforce testing only public interfaces, to prevent test brittleness.
