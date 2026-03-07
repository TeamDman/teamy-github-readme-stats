use arbitrary::Arbitrary;
use eyre::Result;
use facet::Facet;
use figue as args;

/// Generate an SVG or PNG stats card from cached or freshly fetched data.
#[derive(Facet, Arbitrary, Debug, PartialEq)]
#[facet(rename_all = "kebab-case")]
#[expect(clippy::struct_excessive_bools)]
pub struct GenerateArgs {
    /// GitHub username to render.
    #[facet(args::positional)]
    #[arbitrary(default)]
    pub username: String,

    /// Output file path. Supported extensions: .svg, .png
    #[facet(args::positional)]
    #[arbitrary(default)]
    pub output: String,

    /// Include private contribution counts when available.
    #[facet(args::named, default)]
    pub count_private: bool,

    /// Query the search API for all-time commit counts.
    #[facet(args::named, default)]
    pub include_all_commits: bool,

    /// Force a refetch instead of using cached data.
    #[facet(args::named, default)]
    pub refresh: bool,

    /// GitHub token to use instead of the environment.
    #[facet(args::named)]
    pub github_token: Option<String>,

    /// Card theme to use: dark or light.
    #[facet(args::named)]
    pub theme: Option<String>,

    /// Open the generated file after writing it.
    #[facet(args::named, default)]
    pub open: bool,
}

impl GenerateArgs {
    /// # Errors
    ///
    /// This function will return an error if stats cannot be loaded or the output file cannot be written.
    pub async fn invoke(self) -> Result<()> {
        let request = self.request();
        let output_path = std::path::PathBuf::from(self.output.trim());
        let record = if self.refresh {
            self.fetch_and_cache(&request).await?
        } else if let Some(record) =
            crate::cache_store::read_stats_record(&crate::paths::CACHE_DIR, &request).await?
        {
            record
        } else {
            self.fetch_and_cache(&request).await?
        };

        let theme = crate::card_render::CardTheme::from_optional_name(self.theme.as_deref())?;
        crate::card_render::write_card(&output_path, &record, theme)?;
        println!("Generated {}", output_path.display());

        if self.open {
            open::that_detached(&output_path)?;
        }

        Ok(())
    }

    async fn fetch_and_cache(
        &self,
        request: &crate::models::StatsRequest,
    ) -> Result<crate::models::CachedStatsRecord> {
        let client = crate::github_api::GitHubClient::new(self.github_token.clone())?;
        let stats = client.fetch_stats(request).await?;
        let record = crate::models::CachedStatsRecord::new(request.clone(), stats);
        crate::cache_store::write_stats_record(&crate::paths::CACHE_DIR, &record).await?;
        Ok(record)
    }

    fn request(&self) -> crate::models::StatsRequest {
        crate::models::StatsRequest {
            username: self.username.trim().to_owned(),
            count_private: self.count_private,
            include_all_commits: self.include_all_commits,
        }
    }
}
