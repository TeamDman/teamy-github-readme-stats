use azure_identity::DeveloperToolsCredential;
use azure_identity::WorkloadIdentityCredential;
use azure_security_keyvault_secrets::SecretClient;
use eyre::Result;
use eyre::WrapErr;

const DEFAULT_SECRET_NAME: &str = "github-token";

/// Resolve the GitHub token from CLI args, environment, or Azure Key Vault.
///
/// # Errors
///
/// This function will return an error if no token source is available or the selected source fails.
pub async fn resolve_github_token(
    cli_token: Option<String>,
    key_vault_name: Option<String>,
    key_vault_secret_name: Option<String>,
) -> Result<String> {
    if let Ok(token) = crate::github_api::resolve_github_token_from_env(cli_token) {
        return Ok(token);
    }

    let vault_name = key_vault_name
        .or_else(|| std::env::var("KEY_VAULT_NAME").ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            eyre::eyre!(
                "A GitHub token is required. Set GITHUB_TOKEN or GH_TOKEN, pass --github-token, or configure KEY_VAULT_NAME."
            )
        })?;

    let secret_name = key_vault_secret_name
        .or_else(|| std::env::var("GITHUB_TOKEN_SECRET_NAME").ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_SECRET_NAME.to_owned());

    read_secret_from_key_vault(&vault_name, &secret_name).await
}

async fn read_secret_from_key_vault(vault_name: &str, secret_name: &str) -> Result<String> {
    let vault_url = format!("https://{vault_name}.vault.azure.net/");

    if std::env::var_os("AZURE_FEDERATED_TOKEN_FILE").is_some() {
        let credential = WorkloadIdentityCredential::new(None)
            .wrap_err("Failed to initialize Azure workload identity credential")?;
        let client = SecretClient::new(&vault_url, credential, None)
            .wrap_err_with(|| format!("Failed to create Key Vault client for {vault_url}"))?;
        return read_secret_with_client(client, secret_name).await;
    }

    let credential = DeveloperToolsCredential::new(None)
        .wrap_err("Failed to initialize Azure developer tools credential")?;
    let client = SecretClient::new(&vault_url, credential, None)
        .wrap_err_with(|| format!("Failed to create Key Vault client for {vault_url}"))?;
    read_secret_with_client(client, secret_name).await
}

async fn read_secret_with_client(client: SecretClient, secret_name: &str) -> Result<String> {
    let secret = client
        .get_secret(secret_name, None)
        .await
        .wrap_err_with(|| format!("Failed to read Key Vault secret '{secret_name}'"))?
        .into_model()
        .wrap_err_with(|| format!("Failed to deserialize Key Vault secret '{secret_name}'"))?;

    secret
        .value
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| eyre::eyre!("Key Vault secret '{secret_name}' was empty"))
}
