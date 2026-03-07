/// Compute a normal CDF approximation matching the original JavaScript implementation.
#[must_use]
fn normalcdf(mean: f64, sigma: f64, to: f64) -> f64 {
    let z = (to - mean) / f64::sqrt(2.0 * sigma * sigma);
    let t = 1.0 / (1.0 + 0.327_591_1 * z.abs());
    let a1 = 0.254_829_592;
    let a2 = -0.284_496_736;
    let a3 = 1.421_413_741;
    let a4 = -1.453_152_027;
    let a5 = 1.061_405_429;
    let erf = 1.0 - (((((a5 * t + a4) * t + a3) * t + a2) * t + a1) * t * f64::exp(-z * z));
    let sign = if z < 0.0 { -1.0 } else { 1.0 };
    0.5 * (1.0 + sign * erf)
}

#[must_use]
#[expect(clippy::cast_precision_loss)]
pub fn calculate_rank(
    total_repos: u64,
    total_commits: u64,
    contributions: u64,
    followers: u64,
    prs: u64,
    issues: u64,
    stargazers: u64,
) -> crate::models::Rank {
    const COMMITS_OFFSET: f64 = 1.65;
    const CONTRIBS_OFFSET: f64 = 1.65;
    const ISSUES_OFFSET: f64 = 1.0;
    const STARS_OFFSET: f64 = 0.75;
    const PRS_OFFSET: f64 = 0.5;
    const FOLLOWERS_OFFSET: f64 = 0.45;
    const REPO_OFFSET: f64 = 1.0;

    const ALL_OFFSETS: f64 = CONTRIBS_OFFSET
        + ISSUES_OFFSET
        + STARS_OFFSET
        + PRS_OFFSET
        + FOLLOWERS_OFFSET
        + REPO_OFFSET;

    const RANK_S_VALUE: f64 = 1.0;
    const RANK_DOUBLE_A_VALUE: f64 = 25.0;
    const RANK_A2_VALUE: f64 = 45.0;
    const RANK_A3_VALUE: f64 = 60.0;
    const RANK_B_VALUE: f64 = 100.0;

    const TOTAL_VALUES: f64 = RANK_S_VALUE + RANK_A2_VALUE + RANK_A3_VALUE + RANK_B_VALUE;

    let score = (total_commits as f64 * COMMITS_OFFSET
        + contributions as f64 * CONTRIBS_OFFSET
        + issues as f64 * ISSUES_OFFSET
        + stargazers as f64 * STARS_OFFSET
        + prs as f64 * PRS_OFFSET
        + followers as f64 * FOLLOWERS_OFFSET
        + total_repos as f64 * REPO_OFFSET)
        / 100.0;

    let normalized_score = normalcdf(score, TOTAL_VALUES, ALL_OFFSETS) * 100.0;
    let level = if normalized_score < RANK_S_VALUE {
        "S+"
    } else if normalized_score < RANK_DOUBLE_A_VALUE {
        "S"
    } else if normalized_score < RANK_A2_VALUE {
        "A++"
    } else if normalized_score < RANK_A3_VALUE {
        "A+"
    } else {
        "B+"
    };

    crate::models::Rank {
        level: level.to_owned(),
        score: normalized_score,
    }
}
