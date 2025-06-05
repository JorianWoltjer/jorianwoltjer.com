use std::sync::LazyLock;

use fancy_regex::Regex;
use markdown::message::Message;
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::{SyntaxSet, SyntaxSetBuilder},
    util::LinesWithEndings,
};

static HEADER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<h([1-6])>(.*?)</h[1-6]>").unwrap());
static CODE_BLOCK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<pre><code class="language-(.*?)">([\S\s]*?)</code></pre>"#).unwrap()
});
static IMG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<img ([^>]*?)*src="(.*?)"(.*?) />"#).unwrap());
static VIDEO_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<img src="(.*?)\.mp4"(.*?) />"#).unwrap());
static ANCHOR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<a href="(.*?)">([\S\s]*?)</a>"#).unwrap());
static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(|| {
    let mut syntax_builder = SyntaxSetBuilder::new();
    syntax_builder.add_from_folder("syntaxes", true).unwrap();
    syntax_builder.add_plain_text_syntax();
    syntax_builder.build()
});

fn slugify(title: &str) -> String {
    Regex::new(r"((<.*?>)|(&.*?;)|[^\w])+")
        .unwrap()
        .replace_all(title, "-")
        .trim_matches('-')
        .to_string()
        .to_lowercase()
}

pub fn markdown_to_html(markdown: &str) -> Result<String, Message> {
    let markdown = markdown.replace("\r\n", "\n");
    let mut html = markdown::to_html_with_options(
        &markdown,
        &markdown::Options {
            parse: markdown::ParseOptions::gfm(),
            compile: markdown::CompileOptions {
                allow_dangerous_html: true, // Don't care about Self-XSS, required for <iframe>
                ..markdown::CompileOptions::default()
            },
        },
    )?;

    // Add IDs and links to headings
    // PS. Don't we all love parsing HTML with Regex :)
    html = HEADER_REGEX
        .replace_all(&html, |caps: &fancy_regex::Captures| {
            let level = caps.get(1).unwrap().as_str();
            let title = caps.get(2).unwrap().as_str();
            let id = slugify(title);
            format!(r#"<h{level} id="{id}">{title}</h{level}>"#)
        })
        .to_string();

    // Syntax highlighting in code blocks
    html = CODE_BLOCK_REGEX
        .replace_all(&html, |caps: &fancy_regex::Captures| {
            let mut lang = caps.get(1).unwrap().as_str();
            let wrap = lang.ends_with("+wrap");
            if wrap {
                lang = &lang[..lang.len() - 5]; // Remove +wrap suffix
            }
            let code = caps.get(2).unwrap().as_str();
            let code = html_escape::decode_html_entities(code);

            // Look up language by name
            let syntax = SYNTAXES
                .find_syntax_by_token(lang)
                .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());
            let mut rs_html_generator = ClassedHTMLGenerator::new_with_class_style(
                syntax,
                &SYNTAXES,
                ClassStyle::SpacedPrefixed { prefix: "hl-" },
            );
            for line in LinesWithEndings::from(&code) {
                rs_html_generator
                    .parse_html_for_line_which_includes_newline(line)
                    .unwrap();
            }
            let code = rs_html_generator.finalize();

            format!(
                r#"<pre><code data-lang="{lang}"{}>{code}</code></pre>"#,
                if wrap { " class=\"wrap\"" } else { "" }
            )
        })
        .to_string();
    // Image relative paths
    html = IMG_REGEX
        .replace_all(&html, r#"<img ${1}src="/img/blog/$2"$3 />"#)
        .to_string();
    // Replace .mp4 files with <video> tag
    html = VIDEO_REGEX
        .replace_all(&html, r#"<video controls src="$1.mp4"$2></video>"#)
        .to_string();
    // Make all links external
    html = ANCHOR_REGEX
        .replace_all(&html, r#"<a href="$1" target="_blank">$2</a>"#)
        .to_string();

    Ok(html)
}
