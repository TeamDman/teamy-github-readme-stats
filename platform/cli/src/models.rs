use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

pub const CACHE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatsRequest {
    pub username: String,
    pub count_private: bool,
    pub include_all_commits: bool,
}

impl StatsRequest {
    #[must_use]
    pub fn cache_file_name(&self) -> String {
        format!(
            "{}__private-{}__all-{}.json",
            sanitize_path_segment(&self.username),
            u8::from(self.count_private),
            u8::from(self.include_all_commits)
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedStatsRecord {
    pub schema_version: u32,
    pub fetched_at: DateTime<Utc>,
    pub request: StatsRequest,
    pub stats: GitHubUserStats,
}

impl CachedStatsRecord {
    #[must_use]
    pub fn new(request: StatsRequest, stats: GitHubUserStats) -> Self {
        Self {
            schema_version: CACHE_SCHEMA_VERSION,
            fetched_at: Utc::now(),
            request,
            stats,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GitHubUserStats {
    pub name: String,
    pub login: String,
    pub total_repos: u64,
    pub followers: u64,
    pub total_stars: u64,
    pub total_commits: u64,
    pub total_issues: u64,
    pub total_prs: u64,
    pub contributed_to: u64,
    pub rank: Rank,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rank {
    pub level: String,
    pub score: f64,
}

#[must_use]
pub fn sanitize_path_segment(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();

    if sanitized.is_empty() {
        "unknown".to_owned()
    } else {
        sanitized
    }
}
