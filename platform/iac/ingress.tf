resource "kubernetes_ingress_v1" "frontend" {
  metadata {
    name      = "frontend"
    namespace = kubernetes_namespace_v1.main.metadata.0.name
    annotations = {
      "kubernetes.io/ingress.class" = "nginx"
      "cert-manager.io/issuer"      = "letsencrypt-prod"
    }
  }

  spec {
    ingress_class_name = "nginx"

    tls {
      hosts       = ["github-readme-stats.teamdman.ca"]
      secret_name = "github-readme-stats-teamdman-ca-tls"
    }

    rule {
      host = "github-readme-stats.teamdman.ca"

      http {
        path {
          path      = "/"
          path_type = "Prefix"

          backend {
            service {
              name = kubernetes_service_v1.frontend.metadata.0.name
              port {
                number = kubernetes_service_v1.frontend.spec.0.port.0.port
              }
            }
          }
        }
      }
    }
  }

  depends_on = [kubernetes_manifest.letsencrypt_prod]
}
