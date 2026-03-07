use arbitrary::Arbitrary;
use eyre::Result;
use facet::Facet;

/// Delete the cache files.
#[derive(Facet, Arbitrary, Debug, PartialEq)]
pub struct CacheCleanArgs;

impl CacheCleanArgs {
    /// # Errors
    ///
    /// This function will return an error if removing cached files fails.
    #[expect(clippy::unused_async)]
    pub async fn invoke(self) -> Result<()> {
        let result = crate::paths::clean_cache()?;
        println!(
            "Removed {} cache entr{}",
            result.entries_removed,
            if result.entries_removed == 1 {
                "y"
            } else {
                "ies"
            }
        );
        Ok(())
    }
}
