data "azurerm_kubernetes_cluster" "main" {
  resource_group_name = "CACN-Cluster-Benthic-PROD-RG"
  name                = "Benthic-PROD-AKS"
}

data "kubernetes_service_v1" "ingress_nginx_controller" {
  metadata {
    name      = "ingress-nginx-controller"
    namespace = "ingress-nginx"
  }
}
