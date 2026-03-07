use arbitrary::Arbitrary;
use axum::Router;
use axum::extract::Query;
use axum::extract::State;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use eyre::Result;
use eyre::WrapErr;
use facet::Facet;
use figue as args;
use serde::Deserialize;
use std::net::SocketAddr;

/// Serve animated GitHub stats cards over HTTP.
#[derive(Facet, Arbitrary, Debug, PartialEq)]
#[facet(rename_all = "kebab-case")]
pub struct ServeArgs {
    /// Socket address to bind, for example 0.0.0.0:8080.
    #[facet(args::named)]
    pub listen: Option<String>,

    /// GitHub token to use instead of the environment or Key Vault.
    #[facet(args::named)]
    pub github_token: Option<String>,

    /// Azure Key Vault name used to resolve the GitHub token when env vars are absent.
    #[facet(args::named)]
    pub key_vault_name: Option<String>,

    /// Azure Key Vault secret name holding the GitHub token.
    #[facet(args::named)]
    pub key_vault_secret_name: Option<String>,

    /// Default GitHub username when the request omits ?username=.
    #[facet(args::named)]
    pub default_username: Option<String>,

    /// Default card theme when the request omits ?theme=.
    #[facet(args::named)]
    pub theme: Option<String>,

    /// Force refreshes on every request instead of using the cache when possible.
    #[facet(args::named, default)]
    pub refresh: bool,
}

impl ServeArgs {
    /// # Errors
    ///
    /// This function will return an error if the token cannot be resolved or the HTTP server fails.
    pub async fn invoke(self) -> Result<()> {
        let token = crate::github_token::resolve_github_token(
            self.github_token,
            self.key_vault_name,
            self.key_vault_secret_name,
        )
        .await?;
        let client = crate::github_api::GitHubClient::from_token(&token)?;
        let default_theme =
            crate::card_render::CardTheme::from_optional_name(self.theme.as_deref())?;
        let listen = self.listen.unwrap_or_else(|| "0.0.0.0:8080".to_owned());
        let address: SocketAddr = listen
            .parse()
            .wrap_err_with(|| format!("Invalid listen address '{listen}'"))?;

        let app = Router::new()
            .route("/", get(render_svg))
            .route("/stats.svg", get(render_svg))
            .route("/healthz", get(healthz))
            .with_state(AppState {
                client,
                default_theme,
                default_username: self
                    .default_username
                    .map(|value| value.trim().to_owned())
                    .filter(|value| !value.is_empty()),
                refresh: self.refresh,
            });

        let listener = tokio::net::TcpListener::bind(address)
            .await
            .wrap_err_with(|| format!("Failed to bind {address}"))?;
        tracing::info!(%address, "Serving stats cards");
        axum::serve(listener, app)
            .await
            .wrap_err("HTTP server failed")?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct AppState {
    client: crate::github_api::GitHubClient,
    default_theme: crate::card_render::CardTheme,
    default_username: Option<String>,
    refresh: bool,
}

#[derive(Debug, Deserialize)]
struct SvgQuery {
    username: Option<String>,
    count_private: Option<bool>,
    include_all_commits: Option<bool>,
    refresh: Option<bool>,
    theme: Option<String>,
}

async fn healthz() -> &'static str {
    "ok"
}

async fn render_svg(
    State(state): State<AppState>,
    Query(query): Query<SvgQuery>,
) -> std::result::Result<impl IntoResponse, (StatusCode, String)> {
    let username = query
        .username
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .or_else(|| state.default_username.clone())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Missing required query parameter 'username'".to_owned(),
            )
        })?;

    let request = crate::models::StatsRequest {
        username,
        count_private: query.count_private.unwrap_or(false),
        include_all_commits: query.include_all_commits.unwrap_or(false),
    };
    let theme = match query.theme.as_deref() {
        Some(value) => crate::card_render::CardTheme::from_optional_name(Some(value))
            .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?,
        None => state.default_theme,
    };

    let record = load_record(
        &state.client,
        &request,
        query.refresh.unwrap_or(state.refresh),
    )
    .await
    .map_err(|error| internal_error(&error))?;
    let svg = crate::card_render::render_stats_svg(&record, theme);

    Ok((
        [
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static("image/svg+xml; charset=utf-8"),
            ),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=300"),
            ),
        ],
        svg,
    ))
}

async fn load_record(
    client: &crate::github_api::GitHubClient,
    request: &crate::models::StatsRequest,
    refresh: bool,
) -> Result<crate::models::CachedStatsRecord> {
    if !refresh
        && let Some(record) =
            crate::cache_store::read_stats_record(&crate::paths::CACHE_DIR, request).await?
    {
        return Ok(record);
    }

    let stats = client.fetch_stats(request).await?;
    let record = crate::models::CachedStatsRecord::new(request.clone(), stats);
    crate::cache_store::write_stats_record(&crate::paths::CACHE_DIR, &record).await?;
    Ok(record)
}

fn internal_error(error: &eyre::Report) -> (StatusCode, String) {
    tracing::error!(error = %error, "Request failed");
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
