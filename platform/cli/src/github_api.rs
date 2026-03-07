use eyre::Context;
use eyre::Result;
use eyre::bail;
use serde::Deserialize;

const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_GRAPHQL_URL: &str = "https://api.github.com/graphql";
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Thin GitHub API client for the data required by the card generator.
#[derive(Clone, Debug)]
pub struct GitHubClient {
    http: reqwest::Client,
    token: String,
}

impl GitHubClient {
    /// # Errors
    ///
    /// This function will return an error if no GitHub token is available or the HTTP client cannot be constructed.
    pub fn new(cli_token: Option<String>) -> Result<Self> {
        let token = resolve_github_token(cli_token)?;
        let http = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .wrap_err("Failed to build GitHub HTTP client")?;
        Ok(Self { http, token })
    }

    /// # Errors
    ///
    /// This function will return an error if any GitHub API request fails.
    pub async fn fetch_stats(
        &self,
        request: &crate::models::StatsRequest,
    ) -> Result<crate::models::GitHubUserStats> {
        let user = self.fetch_user_snapshot(&request.username).await?;
        let experimental_total_commits = if request.include_all_commits {
            self.fetch_total_commits(&request.username).await?
        } else {
            0
        };

        let contribution_count = &user.contributions_collection;
        let restricted_contributions = if request.count_private {
            contribution_count.restricted_contributions_count
        } else {
            0
        };
        let total_stars = user
            .repositories
            .nodes
            .iter()
            .map(|repository| repository.stargazer_count)
            .sum();
        let total_commits = contribution_count.total_commit_contributions
            + experimental_total_commits
            + restricted_contributions;

        Ok(crate::models::GitHubUserStats {
            name: user.name.unwrap_or_else(|| user.login.clone()),
            login: user.login,
            total_repos: user.repositories.total_count,
            followers: user.followers.total_count,
            total_stars,
            total_commits,
            total_issues: user.issues.total_count,
            total_prs: user.pull_requests.total_count,
            contributed_to: user.repositories_contributed_to.total_count,
            rank: crate::rank::calculate_rank(
                user.repositories.total_count,
                total_commits,
                user.repositories_contributed_to.total_count,
                user.followers.total_count,
                user.pull_requests.total_count,
                user.issues.total_count,
                total_stars,
            ),
        })
    }

    async fn fetch_user_snapshot(&self, username: &str) -> Result<GraphQlUser> {
        if username.trim().is_empty() {
            bail!("GitHub username cannot be empty");
        }

        let body = GraphQlRequest {
            query: r"
                query userInfo($login: String!) {
                  user(login: $login) {
                    name
                    login
                    contributionsCollection {
                      totalCommitContributions
                      restrictedContributionsCount
                    }
                    repositoriesContributedTo(first: 1, contributionTypes: [COMMIT, ISSUE, PULL_REQUEST, REPOSITORY]) {
                      totalCount
                    }
                    pullRequests(first: 1) {
                      totalCount
                    }
                    issues(first: 1) {
                      totalCount
                    }
                    followers {
                      totalCount
                    }
                    repositories(first: 100, ownerAffiliations: OWNER, isFork: false, orderBy: {direction: DESC, field: STARGAZERS}) {
                      totalCount
                      nodes {
                        stargazerCount
                      }
                    }
                  }
                }
            ",
            variables: GraphQlVariables {
                login: username.to_owned(),
            },
        };

        let response = self
            .http
            .post(GITHUB_GRAPHQL_URL)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await
            .wrap_err("Failed to call the GitHub GraphQL API")?
            .error_for_status()
            .wrap_err("GitHub GraphQL API returned an error status")?;

        let payload: GraphQlEnvelope<GraphQlUserResponse> = response
            .json()
            .await
            .wrap_err("Failed to deserialize the GitHub GraphQL response")?;

        if let Some(errors) = payload.errors
            && let Some(error) = errors.first()
        {
            bail!("GitHub GraphQL error: {}", error.message);
        }

        payload
            .data
            .and_then(|data| data.user)
            .ok_or_else(|| eyre::eyre!("GitHub user '{username}' was not found"))
    }

    async fn fetch_total_commits(&self, username: &str) -> Result<u64> {
        let url = format!("{GITHUB_API_URL}/search/commits");
        let response = self
            .http
            .get(url)
            .bearer_auth(&self.token)
            .query(&[("q", format!("author:{username}"))])
            .header(reqwest::header::ACCEPT, "application/vnd.github+json")
            .send()
            .await
            .wrap_err("Failed to call the GitHub commit search API")?
            .error_for_status()
            .wrap_err("GitHub commit search API returned an error status")?;

        let payload: CommitSearchResponse = response
            .json()
            .await
            .wrap_err("Failed to deserialize the GitHub commit search response")?;
        Ok(payload.total_count)
    }
}

fn resolve_github_token(cli_token: Option<String>) -> Result<String> {
    if let Some(token) = cli_token {
        let trimmed = token.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_owned());
        }
    }

    for key in ["GITHUB_TOKEN", "GH_TOKEN"] {
        if let Ok(token) = std::env::var(key) {
            let trimmed = token.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.to_owned());
            }
        }
    }

    bail!(
        "A GitHub token is required. Set GITHUB_TOKEN or GH_TOKEN, or pass --github-token. A GitHub App is not required for this CLI proof-of-concept."
    )
}

#[derive(Debug, serde::Serialize)]
struct GraphQlRequest {
    query: &'static str,
    variables: GraphQlVariables,
}

#[derive(Debug, serde::Serialize)]
struct GraphQlVariables {
    login: String,
}

#[derive(Debug, Deserialize)]
struct GraphQlEnvelope<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQlError>>,
}

#[derive(Debug, Deserialize)]
struct GraphQlError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct GraphQlUserResponse {
    user: Option<GraphQlUser>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphQlUser {
    name: Option<String>,
    login: String,
    contributions_collection: GraphQlContributionsCollection,
    repositories_contributed_to: GraphQlTotalCount,
    pull_requests: GraphQlTotalCount,
    issues: GraphQlTotalCount,
    followers: GraphQlTotalCount,
    repositories: GraphQlRepositories,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphQlContributionsCollection {
    total_commit_contributions: u64,
    restricted_contributions_count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphQlTotalCount {
    total_count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphQlRepositories {
    total_count: u64,
    nodes: Vec<GraphQlRepositoryNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphQlRepositoryNode {
    stargazer_count: u64,
}

#[derive(Debug, Deserialize)]
struct CommitSearchResponse {
    total_count: u64,
}
