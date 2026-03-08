# teamy-github-readme-stats

[![TeamDman's github stats](https://github-readme-stats.teamdman.ca/stats.svg?username=TeamDman&theme=touhou)](https://github.com/TeamDman/teamy-github-readme-stats.git)

Touhou-styled animated GitHub stats cards, implemented as a Rust CLI and deployed as a hosted SVG service.

The project has two main parts:

- [platform/cli](platform/cli): Rust application that fetches GitHub profile data, renders animated SVG cards, and serves them over HTTP.
- [platform/iac](platform/iac): Terraform stack for Azure, Key Vault, workload identity, Kubernetes, ingress, and TLS.

## Live service

Public endpoint:

- https://github-readme-stats.teamdman.ca/stats.svg?username=TeamDman&theme=touhou

Health check:

- https://github-readme-stats.teamdman.ca/healthz

The live deployment currently serves:

- `image/svg+xml`
- animated card content
- `Cache-Control: public, max-age=300`

## Example README usage

Markdown:

```md
![GitHub stats](https://github-readme-stats.teamdman.ca/stats.svg?username=TeamDman&theme=touhou)
```

HTML:

```html
<img
	alt="GitHub stats"
	src="https://github-readme-stats.teamdman.ca/stats.svg?username=TeamDman&theme=touhou"
/>
```

## Supported query parameters

The hosted endpoint accepts these query parameters:

- `username`: GitHub username to render
- `theme`: `touhou`, `light`, or `dark`
- `count_private`: `true` or `false`
- `include_all_commits`: `true` or `false`
- `refresh`: `true` or `false`

Example:

```text
GET /stats.svg?username=TeamDman&theme=touhou&include_all_commits=true
```

## What the card shows

- stars
- commits
- pull requests
- issues
- repositories contributed to
- computed rank badge

The default visual style is the Touhou-inspired card with animated background art, animated apple, staggered stat reveal, and animated rank circle.

## Repository layout

- [platform/cli](platform/cli): Rust app, Dockerfile, and local CLI docs
- [platform/iac](platform/iac): Terraform for Azure and AKS deployment
- [.github/workflows/publish-cli-container.yml](.github/workflows/publish-cli-container.yml): GHCR publish workflow

## Local development

### Prerequisites

- Rust 1.90+
- Docker Desktop if you want to build or run the container image
- A GitHub token in one of:
	- `GITHUB_TOKEN`
	- `GH_TOKEN`
	- `--github-token ...`

For local Key Vault-backed runs, the app can also resolve the GitHub token from Azure Key Vault using developer credentials.

### CLI quick start

From [platform/cli](platform/cli):

```shell
cargo test
cargo clippy --all-features
cargo run -- fetch TeamDman --include-all-commits
cargo run -- generate TeamDman TeamDman.svg --theme touhou
cargo run -- serve --listen 127.0.0.1:8080 --default-username TeamDman
```

Then open:

```text
http://127.0.0.1:8080/stats.svg?username=TeamDman&theme=touhou
```

More CLI details are in [platform/cli/README.md](platform/cli/README.md).

## Container image

Local build:

```shell
docker build -t teamy-github-readme-stats:dev platform/cli
docker run --rm -p 8080:8080 -e GITHUB_TOKEN=... teamy-github-readme-stats:dev
```

Published image:

- `ghcr.io/teamdman/teamy-github-readme-stats:latest`

The publish workflow is defined in [.github/workflows/publish-cli-container.yml](.github/workflows/publish-cli-container.yml).

## Deployment

The live deployment uses:

- Azure Resource Group: `TEAMY-GitHub-README-Stats-RG`
- Azure Key Vault: `teamygithubreadmestats`
- AKS namespace: `teamy-github-readme-stats`
- workload identity for runtime Key Vault access
- ingress-nginx + cert-manager for HTTPS

Terraform lives in [platform/iac](platform/iac). See [platform/iac/README.md](platform/iac/README.md) for infrastructure notes.

## Notes

- The service is intended for README embedding.
- The app renders SVG directly rather than precomputing static assets.
- The Terraform stack writes the GitHub token into Key Vault using `azurerm_key_vault_secret`, which means the secret value exists in Terraform state.

## License

This repository is licensed under the MPL-2.0. See [LICENSE](LICENSE).

