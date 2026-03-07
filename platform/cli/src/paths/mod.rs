mod app_home;
mod cache;

pub use app_home::*;
pub use cache::*;

pub const APP_HOME_ENV_VAR: &str = "TEAMY_GITHUB_README_STATS_HOME_DIR";
pub const APP_HOME_DIR_NAME: &str = "teamy-github-readme-stats";

pub const APP_CACHE_ENV_VAR: &str = "TEAMY_GITHUB_README_STATS_CACHE_DIR";
pub const APP_CACHE_DIR_NAME: &str = "teamy-github-readme-stats";
