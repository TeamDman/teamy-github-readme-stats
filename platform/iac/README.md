# teamy-github-readme-stats IAC

This Terraform stack creates the Azure and Kubernetes plumbing for hosting `github-readme-stats.teamdman.ca` on `Benthic-PROD-AKS`.

## What it creates

- Azure resource group: `TEAMY-GitHub-README-Stats-RG`
- Azure Key Vault: `teamygithubreadmestats`
- Azure Key Vault secret: `github-token` loaded from 1Password via `op read`
- User-assigned managed identity with Key Vault secret read access
- Kubernetes namespace: `teamy-github-readme-stats`
- Kubernetes service account wired to Azure workload identity
- Namespace-scoped Let's Encrypt issuer using HTTP-01 through `ingress-nginx`
- Runtime Key Vault secret: `github-token`
- Application deployment, service, and ingress for the Rust HTTP server

## DNS

DNS is managed in Namecheap, not Azure.

After apply, create an A record for `github-readme-stats.teamdman.ca` pointing at the `ingress_ip_address` output.

## GitHub token

Terraform reads the GitHub token from 1Password with `op read` and writes it to Key Vault as `github-token`.

This means the secret value is managed by Terraform and will exist in Terraform state.

## Runtime

The deployment is intended to run the published GHCR container and fetch the GitHub token from Key Vault at runtime using Azure workload identity.
