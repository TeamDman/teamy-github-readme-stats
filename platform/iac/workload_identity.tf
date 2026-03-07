resource "azurerm_user_assigned_identity" "main" {
  name                = "teamy-github-readme-stats"
  location            = azurerm_resource_group.main.location
  resource_group_name = azurerm_resource_group.main.name
  tags                = azurerm_resource_group.main.tags
}

resource "azurerm_federated_identity_credential" "main" {
  name                = "teamy-github-readme-stats"
  resource_group_name = azurerm_resource_group.main.name
  parent_id           = azurerm_user_assigned_identity.main.id
  issuer              = data.azurerm_kubernetes_cluster.main.oidc_issuer_url
  subject             = "system:serviceaccount:teamy-github-readme-stats:teamy-github-readme-stats"
  audience            = ["api://AzureADTokenExchange"]
}

resource "azurerm_role_assignment" "key_vault_secrets_user" {
  scope                = azurerm_key_vault.main.id
  role_definition_name = "Key Vault Secrets User"
  principal_id         = azurerm_user_assigned_identity.main.principal_id
}

resource "kubernetes_service_account_v1" "main" {
  metadata {
    name      = "teamy-github-readme-stats"
    namespace = kubernetes_namespace_v1.main.metadata.0.name
    labels = {
      "azure.workload.identity/use" = "true"
      "app.kubernetes.io/name"      = "teamy-github-readme-stats"
    }
    annotations = {
      "azure.workload.identity/client-id" = azurerm_user_assigned_identity.main.client_id
      "azure.workload.identity/tenant-id" = data.azurerm_client_config.main.tenant_id
    }
  }
}
