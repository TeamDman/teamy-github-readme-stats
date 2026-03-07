resource "kubernetes_namespace_v1" "main" {
  metadata {
    name = "teamy-github-readme-stats"
    labels = {
      "app.kubernetes.io/name" = "teamy-github-readme-stats"
    }
  }
}
