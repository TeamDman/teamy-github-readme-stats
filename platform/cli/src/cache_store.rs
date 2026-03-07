use eyre::Result;
use eyre::WrapErr;

const STATS_CACHE_DIR_NAME: &str = "stats";

/// Read a cached stats record if it exists.
///
/// # Errors
///
/// This function will return an error if the cache file exists but cannot be read or parsed.
pub async fn read_stats_record(
    cache_home: &crate::paths::CacheHome,
    request: &crate::models::StatsRequest,
) -> Result<Option<crate::models::CachedStatsRecord>> {
    let path = stats_cache_path(cache_home, request);
    if !tokio::fs::try_exists(&path).await? {
        return Ok(None);
    }

    let content = tokio::fs::read_to_string(&path)
        .await
        .wrap_err_with(|| format!("Failed to read {}", path.display()))?;
    let record = serde_json::from_str(&content)
        .wrap_err_with(|| format!("Failed to parse {}", path.display()))?;
    Ok(Some(record))
}

/// Persist a stats record to the cache.
///
/// # Errors
///
/// This function will return an error if directories cannot be created or the file cannot be written.
pub async fn write_stats_record(
    cache_home: &crate::paths::CacheHome,
    record: &crate::models::CachedStatsRecord,
) -> Result<std::path::PathBuf> {
    let path = stats_cache_path(cache_home, &record.request);
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .wrap_err_with(|| format!("Failed to create {}", parent.display()))?;
    }

    let json = serde_json::to_string_pretty(record)?;
    tokio::fs::write(&path, json)
        .await
        .wrap_err_with(|| format!("Failed to write {}", path.display()))?;
    Ok(path)
}

#[must_use]
pub fn stats_cache_path(
    cache_home: &crate::paths::CacheHome,
    request: &crate::models::StatsRequest,
) -> std::path::PathBuf {
    cache_home
        .0
        .join(STATS_CACHE_DIR_NAME)
        .join(request.cache_file_name())
}
