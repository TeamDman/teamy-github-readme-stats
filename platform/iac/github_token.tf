data "external" "github_token" {
  program = [
    "pwsh",
    "-Command",
    <<-EOT
      $ErrorActionPreference = 'Stop'
      $token = op read 'op://Private/teamy-github-readme-stats access token/credential' --no-newline
      if ([string]::IsNullOrWhiteSpace($token)) {
        throw '1Password returned an empty GitHub token'
      }

      @{ value = $token } | ConvertTo-Json -Compress
    EOT
  ]
}

resource "azurerm_role_assignment" "terraform_key_vault_secrets_officer" {
  scope                = azurerm_key_vault.main.id
  role_definition_name = "Key Vault Secrets Officer"
  principal_id         = data.azurerm_client_config.main.object_id
}

resource "azurerm_key_vault_secret" "github_token" {
  name         = "github-token"
  value        = data.external.github_token.result.value
  key_vault_id = azurerm_key_vault.main.id
  content_type = "GitHub API token"

  depends_on = [
    azurerm_key_vault.main,
    azurerm_role_assignment.terraform_key_vault_secrets_officer,
  ]
}
