use eyre::Result;
use eyre::WrapErr;
use eyre::bail;
use resvg::tiny_skia;
use std::f64::consts::PI;
use std::fmt::Write as _;

const CARD_WIDTH: u32 = 495;
const CARD_HEIGHT: u32 = 195;
const RANK_RADIUS: f64 = 40.0;
const TOUHOU_PATH: &str = r#"M2381 3155 c-90 -86 -91 -91 -66 -185 l15 -55 -32 -28 c-88 -77 -139
-131 -157 -165 -34 -67 -63 -175 -63 -237 1 -63 33 -172 51 -168 5 2 15 -14
22 -35 20 -62 29 -75 44 -62 9 8 14 7 19 -5 5 -14 21 -23 74 -39 21 -7 21 -6
12 21 -10 27 -9 27 20 -7 36 -43 36 -48 5 -95 -29 -42 -23 -59 26 -70 16 -4
34 -15 39 -26 22 -41 151 -159 173 -159 13 0 28 -5 33 -10 13 -13 64 4 64 20
0 5 6 10 14 10 8 0 17 6 19 13 3 6 6 -12 6 -40 l1 -53 -77 -75 c-57 -54 -85
-74 -105 -75 -39 0 -86 -44 -99 -93 -6 -23 -24 -57 -41 -75 -38 -44 -49 -75
-48 -131 1 -25 -6 -68 -16 -96 -9 -27 -31 -92 -48 -144 -17 -52 -36 -96 -42
-98 -17 -6 -150 294 -169 382 -10 44 -26 102 -36 128 -11 26 -19 52 -19 56 0
5 24 12 53 16 121 15 313 48 327 55 31 18 -65 22 -242 11 -154 -9 -185 -8
-198 4 -23 21 -102 55 -128 55 -12 0 -39 -9 -59 -20 -21 -11 -43 -20 -50 -20
-7 0 -20 -11 -29 -25 -18 -27 -82 -51 -98 -35 -5 5 -39 60 -76 122 l-67 113
-39 -1 c-107 -2 -142 -11 -168 -40 -14 -16 -26 -33 -26 -39 0 -5 44 -84 99
-175 76 -126 96 -168 89 -180 -7 -11 -3 -37 15 -90 14 -41 27 -94 30 -117 6
-52 54 -174 76 -190 9 -7 23 -13 31 -13 9 0 31 -8 50 -19 19 -10 91 -49 160
-85 129 -69 170 -99 170 -124 0 -39 46 -99 171 -222 l137 -136 -11 -51 c-7
-28 -22 -62 -33 -75 -12 -13 -27 -33 -33 -45 -16 -31 -14 -116 4 -139 8 -10
15 -36 15 -56 l0 -38 611 0 610 0 -7 23 c-21 68 -103 167 -139 167 -26 0 -26
-2 -51 185 -8 66 -16 198 -17 293 -2 170 -2 174 -26 193 -14 12 -31 17 -42 14
-15 -5 -19 -1 -19 18 0 34 7 37 52 21 43 -15 96 -18 147 -8 25 5 30 10 26 25
-6 17 0 19 53 19 54 0 60 2 55 19 -4 16 3 19 55 25 l60 7 6 38 c3 21 15 49 26
62 l19 24 1 -54 c0 -48 2 -53 20 -48 25 6 25 6 0 -117 -14 -67 -17 -103 -10
-110 5 -5 11 -8 13 -5 61 86 82 101 66 47 -13 -43 -24 -228 -14 -228 4 0 13 7
19 15 8 12 10 -7 8 -76 -4 -113 6 -129 50 -79 16 19 28 28 25 19 -3 -8 -8 -46
-12 -82 -9 -79 0 -84 60 -37 21 17 39 29 41 27 2 -1 -3 -30 -11 -62 -19 -82
-19 -88 6 -82 18 5 20 2 15 -18 -8 -32 10 -41 36 -18 11 10 23 16 26 13 3 -3
12 13 20 35 24 68 33 88 38 83 2 -3 0 -17 -6 -32 -15 -41 -49 -272 -43 -295 5
-21 11 -21 284 -21 l279 0 0 778 0 779 -68 12 c-89 16 -148 44 -280 133 -134
90 -324 277 -401 396 -29 45 -59 82 -66 82 -8 0 -16 -13 -20 -29 -3 -16 -21
-45 -39 -64 -18 -19 -48 -59 -66 -88 -74 -118 -334 -399 -369 -399 -4 0 -17 9
-29 20 -19 18 -22 31 -21 88 2 106 9 127 43 127 18 0 33 -7 40 -20 6 -11 12
-22 14 -24 3 -6 31 33 50 68 6 13 12 51 13 85 l1 61 9 -44 c10 -53 17 -62 30
-38 6 9 17 17 26 17 10 0 13 5 8 18 -17 41 -16 49 5 55 20 5 21 7 6 35 -8 16
-12 39 -9 51 5 18 11 21 30 16 l24 -6 -25 38 c-27 39 -35 66 -14 45 42 -42 12
37 -54 141 -28 45 -44 88 -68 193 -44 185 -77 248 -190 360 -102 100 -192 157
-265 167 -60 9 -65 13 -85 79 -15 51 -16 52 -75 70 -33 10 -90 21 -126 25
l-66 6 -82 -78z m701 -1395 c2 -22 2 -40 1 -40 -1 0 -10 11 -19 25 -13 20 -14
28 -4 40 17 21 17 20 22 -25z m-1347 -370 c3 -11 24 -43 46 -71 23 -28 47 -67
56 -87 l15 -36 -34 26 c-29 23 -66 74 -113 161 -15 25 -14 27 4 27 11 0 22 -9
26 -20z m1364 -745 c13 -107 15 -263 3 -250 -5 6 -21 40 -36 76 -28 66 -28 68
-16 160 20 163 30 166 49 14z m1252 -457 c-9 -31 -9 -31 -10 -6 -1 30 11 67
16 51 2 -6 0 -26 -6 -45z"#;
const TOUHOU_APPLE_PATH: &str = r#"M1087 1760 c-15 -12 -48 -65 -73 -118 -25 -53 -47 -98 -49 -100 -1
-1 -76 15 -166 38 -268 66 -355 53 -480 -70 -142 -139 -218 -387 -203 -655 15
-266 86 -439 217 -529 44 -30 66 -38 129 -45 58 -7 84 -15 110 -34 19 -15 61
-39 93 -54 51 -23 76 -28 167 -31 105 -4 109 -3 196 33 234 97 417 316 524
625 41 118 49 260 20 342 -49 139 -224 261 -442 309 -74 16 -84 24 -66 58 8
14 21 42 30 61 25 55 66 126 66 115 0 -12 20 16 20 29 0 4 -8 6 -18 4 -13 -4
-18 0 -18 16 1 28 -25 31 -57 6z"#;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardTheme {
    Touhou,
    Dark,
    Light,
}

impl CardTheme {
    /// # Errors
    ///
    /// This function will return an error if an unsupported theme name is provided.
    pub fn from_optional_name(value: Option<&str>) -> Result<Self> {
        match value.map(str::trim).filter(|value| !value.is_empty()) {
            None => Ok(Self::Touhou),
            Some(value) if value.eq_ignore_ascii_case("touhou") => Ok(Self::Touhou),
            Some(value) if value.eq_ignore_ascii_case("dark") => Ok(Self::Dark),
            Some(value) if value.eq_ignore_ascii_case("light") => Ok(Self::Light),
            Some(value) => {
                bail!("Unsupported theme '{value}'. Expected 'touhou', 'dark', or 'light'.")
            }
        }
    }

    #[must_use]
    fn palette(self) -> Palette {
        match self {
            Self::Touhou => Palette {
                background: "#ffffff",
                border: "#e4e2e2",
                title: "#2f80ed",
                text: "#434d58",
                value: "#434d58",
                rank_circle: "#2f80ed",
                art_fill: "#000000",
                show_touhou_background: true,
            },
            Self::Dark => Palette {
                background: "#0d1117",
                border: "#30363d",
                title: "#58a6ff",
                text: "#c9d1d9",
                value: "#c9d1d9",
                rank_circle: "#58a6ff",
                art_fill: "#ffffff",
                show_touhou_background: false,
            },
            Self::Light => Palette {
                background: "#ffffff",
                border: "#cbd5e1",
                title: "#0f172a",
                text: "#1e293b",
                value: "#1e293b",
                rank_circle: "#2563eb",
                art_fill: "#000000",
                show_touhou_background: false,
            },
        }
    }
}

struct Palette {
    background: &'static str,
    border: &'static str,
    title: &'static str,
    text: &'static str,
    value: &'static str,
    rank_circle: &'static str,
    art_fill: &'static str,
    show_touhou_background: bool,
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
        Some("png") => {
            let options = usvg::Options::default();
            let tree = usvg::Tree::from_str(&svg, &options)
                .wrap_err("Generated SVG failed validation for PNG rendering")?;
            write_png(output_path, &tree)?;
        }
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
    let apostrophe = if stats.name.ends_with('x') || stats.name.ends_with('s') {
        ""
    } else {
        "s"
    };
    let title = format!("{}'{} GitHub Stats", stats.name, apostrophe);
    let commit_label = if record.request.include_all_commits {
        "Commits (all time)"
    } else {
        "Commits (this year)"
    };
    let rows = [
        ("Stars", format_metric(stats.total_stars)),
        (commit_label, format_metric(stats.total_commits)),
        ("Pull requests", format_metric(stats.total_prs)),
        ("Issues", format_metric(stats.total_issues)),
        ("Contributed to", format_metric(stats.contributed_to)),
    ];

    let mut rows_svg = String::new();
    for (index, (label, value)) in rows.iter().enumerate() {
        let animation_delay_ms = (index + 3) * 150;
        let y = index * 25;
        let _ = write!(
            rows_svg,
            "<g class=\"stagger\" style=\"animation-delay: {animation_delay_ms}ms\" transform=\"translate(25, {y})\"><text class=\"stat bold\" y=\"12.5\">{}:</text><text class=\"stat value\" x=\"150\" y=\"12.5\">{}</text></g>",
            escape_xml(label),
            escape_xml(value)
        );
    }

    let progress = (100.0 - stats.rank.score).clamp(0.0, 100.0);
    let circumference = PI * (RANK_RADIUS * 2.0);
    let final_dash_offset = calculate_circle_progress(progress);
    let rank_circle = format!(
        "<g data-testid=\"rank-circle\" transform=\"translate(370, 47.5)\"><circle class=\"rank-circle-rim\" cx=\"-10\" cy=\"8\" r=\"40\" /><circle class=\"rank-circle\" cx=\"-10\" cy=\"8\" r=\"40\" transform=\"rotate(-90 -10 8)\" /><g class=\"rank-text\"><text x=\"{}\" y=\"0\" alignment-baseline=\"central\" dominant-baseline=\"central\" text-anchor=\"middle\">{}</text></g></g>",
        if stats.rank.level.chars().count() == 1 {
            "-4"
        } else {
            "0"
        },
        escape_xml(&stats.rank.level)
    );
    let background_art = if palette.show_touhou_background {
        render_touhou_background(palette.art_fill)
    } else {
        String::new()
    };

    format!(
        "<svg width=\"{CARD_WIDTH}\" height=\"{CARD_HEIGHT}\" viewBox=\"0 0 {CARD_WIDTH} {CARD_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-labelledby=\"title desc\">\
<title id=\"title\">{svg_title}</title>\
<desc id=\"desc\">Animated GitHub stats card for {login}, with Touhou-inspired background art.</desc>\
<style>\
  .header {{ font: 600 18px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {title_color}; animation: fadeInAnimation 0.8s ease-in-out forwards; }}\
  .stat {{ font: 600 14px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {text_color}; }}\
  .value {{ fill: {value_color}; }}\
  .bold {{ font-weight: 700; }}\
  .stagger {{ opacity: 0; animation: fadeInAnimation 0.3s ease-in-out forwards; }}\
  .rank-text {{ font: 800 24px 'Segoe UI', Ubuntu, Sans-Serif; fill: {text_color}; animation: scaleInAnimation 0.3s ease-in-out forwards; }}\
    .rank-circle-rim {{ stroke: {rank_circle_color}; fill: none; stroke-width: 6; opacity: 0.2; }}\
    .rank-circle {{ stroke: {rank_circle_color}; stroke-dasharray: {circumference}; stroke-dashoffset: {circumference}; fill: none; stroke-width: 6; stroke-linecap: round; opacity: 0.8; animation: rankAnimation 1s forwards ease-in-out; }}\
  @keyframes scaleInAnimation {{ from {{ transform: translate(-5px, 5px) scale(0); }} to {{ transform: translate(-5px, 5px) scale(1); }} }}\
  @keyframes fadeInAnimation {{ from {{ opacity: 0; }} to {{ opacity: 1; }} }}\
  @keyframes rankAnimation {{ from {{ stroke-dashoffset: {circumference}; }} to {{ stroke-dashoffset: {final_dash_offset}; }} }}\
</style>\
<rect data-testid=\"card-bg\" x=\"0.5\" y=\"0.5\" rx=\"4.5\" height=\"99.5%\" stroke=\"{border}\" width=\"494\" fill=\"{background_color}\" stroke-opacity=\"1\"/>\
{background_art}\
<rect data-testid=\"card-bg2\" x=\"0.5\" y=\"0.5\" rx=\"4.5\" height=\"99.5%\" stroke=\"{border}\" width=\"494\" stroke-opacity=\"1\" fill=\"none\"/>\
<g data-testid=\"card-title\" transform=\"translate(25, 35)\">\
  <text x=\"0\" y=\"0\" class=\"header\">{svg_title}</text>\
</g>\
{rank_circle}\
<g data-testid=\"main-card-body\" transform=\"translate(0, 55)\">\
  <svg x=\"0\" y=\"0\">{rows_svg}</svg>\
</g>\
</svg>",
        svg_title = escape_xml(&title),
        login = escape_xml(&stats.login),
        rows_svg = rows_svg,
        rank_circle = rank_circle,
        background_art = background_art,
        background_color = palette.background,
        border = palette.border,
        title_color = palette.title,
        text_color = palette.text,
        value_color = palette.value,
        rank_circle_color = palette.rank_circle,
        circumference = circumference,
        final_dash_offset = final_dash_offset,
    )
}

#[must_use]
fn render_touhou_background(art_fill: &str) -> String {
    format!(
        "<g data-testid=\"touhou\" transform=\"translate(290.000000,210.000000) scale(0.050000,-0.050000)\" fill=\"{art_fill}\" fill-opacity=\"0.14\" stroke=\"none\"><path d=\"{TOUHOU_PATH}\"/></g><g><g data-testid=\"touhou_apple\" transform=\"translate(0 0) scale(0.020000,-0.020000)\" fill=\"{art_fill}\" fill-opacity=\"0.14\" stroke=\"none\"><path d=\"{TOUHOU_APPLE_PATH}\"/></g><animateTransform attributeName=\"transform\" type=\"translate\" additive=\"sum\" keyTimes=\"0;0.2;0.5;0.8;1\" values=\"280,75;280,78;280,82;280,78;280,75\" keySplines=\".42 0 1 1;.42 0 1 1;.42 0 1 1;.42 0 1 1;.42 0 1 1\" begin=\"0s\" dur=\"2s\" repeatCount=\"indefinite\" /></g>",
    )
}

#[must_use]
fn calculate_circle_progress(value: f64) -> f64 {
    let clamped = value.clamp(0.0, 100.0);
    let circumference = PI * (RANK_RADIUS * 2.0);
    ((100.0 - clamped) / 100.0) * circumference
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
