use lazy_static::lazy_static;
use markdown::message::Message;
use regex::{Regex, RegexBuilder};
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

lazy_static! {
    static ref HEADER_REGEX: Regex = Regex::new(r"<h([1-6])>(.*?)</h[1-6]>").unwrap();
    static ref CODE_BLOCK_REGEX: Regex =
        RegexBuilder::new(r#"<pre><code class="language-(\w+)">(.*?)</code></pre>"#)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
    static ref IMG_REGEX: Regex = Regex::new(r#"<img ([^>]*?)*src="(.*?)"(.*?) />"#).unwrap();
    static ref SYNTAXES: SyntaxSet = SyntaxSet::load_defaults_newlines();
}

fn slugify(title: &str) -> String {
    Regex::new(r"((<.*?>)|(&.*?;)|[^\w])+")
        .unwrap()
        .replace_all(title, "-")
        .trim_matches('-')
        .to_string()
        .to_lowercase()
}

pub fn markdown_to_html(markdown: &str) -> Result<String, Message> {
    let mut html = markdown::to_html_with_options(
        markdown,
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
        .replace_all(&html, |caps: &regex::Captures| {
            let level = caps.get(1).unwrap().as_str();
            let title = caps.get(2).unwrap().as_str();
            let id = slugify(title);
            format!(r#"<h{level} id="{id}">{title}</h{level}>"#)
        })
        .to_string();

    // Syntax highlighting in code blocks
    html = CODE_BLOCK_REGEX
        .replace_all(&html, |caps: &regex::Captures| {
            let lang = caps.get(1).unwrap().as_str();
            let code = caps.get(2).unwrap().as_str();
            let code = html_escape::decode_html_entities(code);

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

            format!(r#"<pre><code data-lang="{lang}">{code}</code></pre>"#)
        })
        .to_string();

    // Image relative paths and wrap in <picture> tag
    html = IMG_REGEX
        .replace_all(&html, r#"<img ${1}src="/img/blog/$2"$3 />"#)
        .to_string();

    Ok(html)
}
