use lazy_static::lazy_static;
use regex::Regex;

pub fn title_to_id(title: &str) -> String {
    Regex::new(r"((<.*?>)|(&.*?;)|[^\w])+")
        .unwrap()
        .replace_all(title, "")
        .to_string()
        .replace(' ', "-")
        .to_lowercase()
}

lazy_static! {
    static ref HEADER_REGEX: Regex = Regex::new(r"<(h[1-6])>(.*?)</(h[1-6])>").unwrap();
    static ref YT_IMG_REGEX: Regex = Regex::new(r#"<img src="(?:https?://(?:www\.)?youtube\.com/watch\?v=|https?://youtu\.be/)([A-Za-z0-9_\-]+)" alt="([^"]*)"\s?/>"#).unwrap();
    static ref CODEBLOCK_REGEX: Regex = Regex::new(r#"(?s)<pre><code class="language-(.*?)">(.*?)<\/code><\/pre>"#).unwrap();
}

pub fn markdown_to_html(markdown: &str) -> Result<String, String> {
    let mut html = markdown::to_html_with_options(markdown, &markdown::Options::gfm())?;
    // Don't we all love parsing HTML with Regex :)

    // Add IDs to headings
    html = HEADER_REGEX
        .replace_all(&html, |caps: &regex::Captures| {
            let title = caps.get(2).unwrap().as_str();
            let id = title_to_id(title);
            format!("<{} id=\"{}\">{}</{}>", &caps[1], id, title, &caps[3])
        })
        .to_string();
    // Convert ![](youtube.com) to <iframe> tags
    html = YT_IMG_REGEX
        .replace_all(&html, r#"<iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/$1" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>"#)
        .to_string();
    // Style code blocks
    html = CODEBLOCK_REGEX
        .replace_all(&html, r#"<div class="code-block"><p>$1<a class="copy" data-bs-toggle="tooltip" data-bs-placement="top" title="Copied!"><i class="fa-solid fa-copy"></i></a></p><pre><code class="hljs language-$1">$2</code></pre></div>"#)
        .to_string();

    Ok(html)
}
