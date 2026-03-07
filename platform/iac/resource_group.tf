resource "azurerm_resource_group" "main" {
  name     = "TEAMY-GitHub-README-Stats-RG"
  location = "canadacentral"
  tags = {
    environment = "Production"
    managed-by  = "terraform"
    workload    = "teamy-github-readme-stats"
  }
}
