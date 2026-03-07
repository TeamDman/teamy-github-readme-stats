output "ingress_ip_address" {
  value       = data.kubernetes_service_v1.ingress_nginx_controller.status.0.load_balancer.0.ingress.0.ip
  description = "Point github-readme-stats.teamdman.ca at this IP in Namecheap."
}

output "key_vault_name" {
  value       = azurerm_key_vault.main.name
  description = "Store the GitHub API token here, for example as secret name github-token."
}

output "workload_identity_client_id" {
  value       = azurerm_user_assigned_identity.main.client_id
  description = "This client ID is already wired to the Kubernetes service account."
}

output "manual_dns_instruction" {
  value       = "Create an A record in Namecheap for github-readme-stats.teamdman.ca pointing to the ingress_ip_address output."
  description = "Namecheap is the DNS provider, so DNS is managed outside Azure in this stack."
}
