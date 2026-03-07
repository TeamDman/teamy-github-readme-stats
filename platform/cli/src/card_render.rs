use eyre::Result;
use eyre::WrapErr;
use eyre::bail;
use resvg::tiny_skia;
use std::fmt::Write as _;

const CARD_WIDTH: u32 = 495;
const CARD_HEIGHT: u32 = 195;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardTheme {
    Dark,
    Light,
}

impl CardTheme {
    /// # Errors
    ///
    /// This function will return an error if an unsupported theme name is provided.
    pub fn from_optional_name(value: Option<&str>) -> Result<Self> {
        match value.map(str::trim).filter(|value| !value.is_empty()) {
            None => Ok(Self::Dark),
            Some(value) if value.eq_ignore_ascii_case("dark") => Ok(Self::Dark),
            Some(value) if value.eq_ignore_ascii_case("light") => Ok(Self::Light),
            Some(value) => bail!("Unsupported theme '{value}'. Expected 'dark' or 'light'."),
        }
    }

    #[must_use]
    fn palette(self) -> Palette {
        match self {
            Self::Dark => Palette {
                canvas: "#0d1117",
                card_start: "#111827",
                card_end: "#1f2937",
                border: "#30363d",
                title: "#58a6ff",
                text: "#c9d1d9",
                muted: "#8b949e",
                accent: "#f78166",
                accent_soft: "#1f6feb",
                badge_fill: "#161b22",
                badge_stroke: "#388bfd",
            },
            Self::Light => Palette {
                canvas: "#ffffff",
                card_start: "#f8fafc",
                card_end: "#e2e8f0",
                border: "#cbd5e1",
                title: "#0f172a",
                text: "#1e293b",
                muted: "#475569",
                accent: "#7c3aed",
                accent_soft: "#2563eb",
                badge_fill: "#ffffff",
                badge_stroke: "#7c3aed",
            },
        }
    }
}

struct Palette {
    canvas: &'static str,
    card_start: &'static str,
    card_end: &'static str,
    border: &'static str,
    title: &'static str,
    text: &'static str,
    muted: &'static str,
    accent: &'static str,
    accent_soft: &'static str,
    badge_fill: &'static str,
    badge_stroke: &'static str,
}

/// Render and write a card to either SVG or PNG based on the output extension.
///
/// # Errors
///
/// This function will return an error if the SVG cannot be validated or the output file cannot be written.
pub fn write_card(
    output_path: &std::path::Path,
    record: &crate::models::CachedStatsRecord,
    theme: CardTheme,
) -> Result<()> {
    let svg = render_stats_svg(record, theme);
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(&svg, &options).wrap_err("Generated SVG failed validation")?;

    if let Some(parent) = output_path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .wrap_err_with(|| format!("Failed to create {}", parent.display()))?;
    }

    match output_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("svg") | None => std::fs::write(output_path, svg)
            .wrap_err_with(|| format!("Failed to write {}", output_path.display()))?,
        Some("png") => write_png(output_path, &tree)?,
        Some(other) => bail!("Unsupported output extension '.{other}'. Use .svg or .png."),
    }

    Ok(())
}

fn write_png(output_path: &std::path::Path, tree: &usvg::Tree) -> Result<()> {
    let size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or_else(|| eyre::eyre!("Failed to allocate a PNG render surface"))?;
    let mut pixmap_mut = pixmap.as_mut();
    resvg::render(tree, tiny_skia::Transform::default(), &mut pixmap_mut);
    pixmap
        .save_png(output_path)
        .wrap_err_with(|| format!("Failed to write {}", output_path.display()))?;
    Ok(())
}

#[must_use]
pub fn render_stats_svg(record: &crate::models::CachedStatsRecord, theme: CardTheme) -> String {
    let palette = theme.palette();
    let stats = &record.stats;
    let title = format!("{}'s GitHub Stats", stats.name);
    let fetched_at = record.fetched_at.format("%Y-%m-%d %H:%M UTC");
    let commit_label = if record.request.include_all_commits {
        "Commits (all time)"
    } else {
        "Commits (this year)"
    };
    let privacy_note = if record.request.count_private {
        "private included"
    } else {
        "public only"
    };

    let rows = [
        ("Stars", format_metric(stats.total_stars)),
        (commit_label, format_metric(stats.total_commits)),
        ("Pull requests", format_metric(stats.total_prs)),
        ("Issues", format_metric(stats.total_issues)),
        ("Contributed to", format_metric(stats.contributed_to)),
        ("Followers", format_metric(stats.followers)),
    ];
    let row_y_positions = [88, 106, 124, 142, 160, 178];

    let mut rows_svg = String::new();
    for (y, (label, value)) in row_y_positions.into_iter().zip(rows.iter()) {
        let _ = write!(
            rows_svg,
            "<text x=\"32\" y=\"{y}\" class=\"label\">{}</text><text x=\"220\" y=\"{y}\" class=\"value\">{}</text>",
            escape_xml(label),
            escape_xml(value)
        );
    }

    format!(
        "<svg width=\"{CARD_WIDTH}\" height=\"{CARD_HEIGHT}\" viewBox=\"0 0 {CARD_WIDTH} {CARD_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-labelledby=\"title desc\">\
<title id=\"title\">{svg_title}</title>\
<desc id=\"desc\">GitHub stats card for {login}, generated by the local Rust CLI.</desc>\
<defs>\
  <linearGradient id=\"cardGradient\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"1\">\
    <stop offset=\"0%\" stop-color=\"{card_start}\"/>\
    <stop offset=\"100%\" stop-color=\"{card_end}\"/>\
  </linearGradient>\
</defs>\
<style>\
  .title {{ font: 600 22px 'Segoe UI', Ubuntu, Sans-Serif; fill: {title_color}; }}\
  .subtitle {{ font: 500 11px 'Segoe UI', Ubuntu, Sans-Serif; fill: {muted}; }}\
  .label {{ font: 500 13px 'Segoe UI', Ubuntu, Sans-Serif; fill: {text}; }}\
  .value {{ font: 700 13px 'Segoe UI', Ubuntu, Sans-Serif; fill: {accent}; text-anchor: end; }}\
  .badge-label {{ font: 600 11px 'Segoe UI', Ubuntu, Sans-Serif; fill: {muted}; text-anchor: middle; }}\
  .badge-value {{ font: 700 28px 'Segoe UI', Ubuntu, Sans-Serif; fill: {accent_soft}; text-anchor: middle; }}\
  .badge-score {{ font: 500 11px 'Segoe UI', Ubuntu, Sans-Serif; fill: {text}; text-anchor: middle; }}\
</style>\
<rect width=\"{CARD_WIDTH}\" height=\"{CARD_HEIGHT}\" rx=\"18\" fill=\"{canvas}\"/>\
<rect x=\"10\" y=\"10\" width=\"475\" height=\"175\" rx=\"16\" fill=\"url(#cardGradient)\" stroke=\"{border}\"/>\
<text x=\"32\" y=\"46\" class=\"title\">{svg_title}</text>\
<text x=\"32\" y=\"64\" class=\"subtitle\">@{login} • fetched {fetched_at} • {privacy_note}</text>\
{rows_svg}\
<g transform=\"translate(385 103)\">\
  <circle cx=\"0\" cy=\"0\" r=\"44\" fill=\"{badge_fill}\" stroke=\"{badge_stroke}\" stroke-width=\"3\"/>\
  <text x=\"0\" y=\"-10\" class=\"badge-label\">Rank</text>\
  <text x=\"0\" y=\"18\" class=\"badge-value\">{rank_level}</text>\
  <text x=\"0\" y=\"36\" class=\"badge-score\">score {rank_score}</text>\
</g>\
<text x=\"272\" y=\"154\" class=\"subtitle\">Repos: {repos} • Followers: {followers}</text>\
</svg>",
        svg_title = escape_xml(&title),
        login = escape_xml(&stats.login),
        fetched_at = escape_xml(&fetched_at.to_string()),
        privacy_note = escape_xml(privacy_note),
        rows_svg = rows_svg,
        rank_level = escape_xml(&stats.rank.level),
        rank_score = escape_xml(&format!("{:.2}", stats.rank.score)),
        repos = escape_xml(&stats.total_repos.to_string()),
        followers = escape_xml(&stats.followers.to_string()),
        canvas = palette.canvas,
        card_start = palette.card_start,
        card_end = palette.card_end,
        border = palette.border,
        title_color = palette.title,
        muted = palette.muted,
        text = palette.text,
        accent = palette.accent,
        accent_soft = palette.accent_soft,
        badge_fill = palette.badge_fill,
        badge_stroke = palette.badge_stroke,
    )
}

#[must_use]
fn format_metric(value: u64) -> String {
    match value {
        0..=999 => value.to_string(),
        1_000..=999_999 => format_metric_with_suffix(value, 1_000, "k"),
        1_000_000..=999_999_999 => format_metric_with_suffix(value, 1_000_000, "M"),
        _ => format_metric_with_suffix(value, 1_000_000_000, "B"),
    }
}

#[must_use]
fn format_metric_with_suffix(value: u64, divisor: u64, suffix: &str) -> String {
    let whole = value / divisor;
    let tenths = (value % divisor) * 10 / divisor;

    let mut formatted = whole.to_string();
    if tenths > 0 {
        formatted.push('.');
        formatted.push_str(&tenths.to_string());
    }
    formatted.push_str(suffix);
    formatted
}

#[must_use]
fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
