# teamy-github-readme-stats

Local Rust CLI for fetching GitHub profile stats and generating README card assets.

## Current scope

The first milestone is parity with the original JavaScript proof-of-concept for the profile stats card:

- `fetch <username>` grabs the GitHub data needed to build a stats card and stores it in the local cache.
- `generate <username> <output>` renders a card from cached data, or refreshes the data when requested.
- `serve` starts an HTTP server that renders SVG cards on demand for README usage.
- `cache`, `docs`, and `home` remain available as utility commands from the template.

## Token setup

This CLI currently expects a classic personal access token or fine-grained token exposed through one of:

- `GITHUB_TOKEN`
- `GH_TOKEN`
- `--github-token ...`

For this local proof-of-concept, a GitHub App is not required.

## Examples

```shell
teamy-github-readme-stats fetch TeamDman --include-all-commits
teamy-github-readme-stats generate TeamDman stats.svg
teamy-github-readme-stats generate TeamDman stats.png --refresh --theme light
teamy-github-readme-stats serve --listen 0.0.0.0:8080 --default-username TeamDman
```

Sample request:

```text
GET /stats.svg?username=TeamDman&theme=touhou&include_all_commits=true
```

## Notes

- Cache files are written under the platform cache directory in a `stats/` subdirectory.
- `generate` validates the produced SVG and can also rasterize to PNG using the same `usvg`/`resvg` stack used by Graphite's renderer ecosystem.
- `serve` resolves the GitHub token from `GITHUB_TOKEN`, `GH_TOKEN`, or Azure Key Vault when `KEY_VAULT_NAME` is configured.
- Help docs can still be regenerated with `docs write docs/`.