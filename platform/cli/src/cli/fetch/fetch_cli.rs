use arbitrary::Arbitrary;
use eyre::Result;
use facet::Facet;
use figue as args;

/// Fetch GitHub profile stats and persist them into the local cache.
#[derive(Facet, Arbitrary, Debug, PartialEq)]
#[facet(rename_all = "kebab-case")]
pub struct FetchArgs {
    /// GitHub username to fetch.
    #[facet(args::positional)]
    #[arbitrary(default)]
    pub username: String,

    /// Include private contribution counts when available.
    #[facet(args::named, default)]
    pub count_private: bool,

    /// Query the search API for all-time commit counts.
    #[facet(args::named, default)]
    pub include_all_commits: bool,

    /// GitHub token to use instead of the environment.
    #[facet(args::named)]
    pub github_token: Option<String>,

    /// Print the cached JSON record after fetching.
    #[facet(args::named, default)]
    pub stdout: bool,
}

impl FetchArgs {
    /// # Errors
    ///
    /// This function will return an error if GitHub data cannot be fetched or the cache cannot be written.
    pub async fn invoke(self) -> Result<()> {
        let request = self.request();
        let client = crate::github_api::GitHubClient::new(self.github_token)?;
        let stats = client.fetch_stats(&request).await?;
        let record = crate::models::CachedStatsRecord::new(request, stats);
        let cache_path =
            crate::cache_store::write_stats_record(&crate::paths::CACHE_DIR, &record).await?;

        println!(
            "Fetched stats for {} into {}",
            record.request.username,
            cache_path.display()
        );

        if self.stdout {
            println!("{}", serde_json::to_string_pretty(&record)?);
        }

        Ok(())
    }

    fn request(&self) -> crate::models::StatsRequest {
        crate::models::StatsRequest {
            username: self.username.trim().to_owned(),
            count_private: self.count_private,
            include_all_commits: self.include_all_commits,
        }
    }
}
