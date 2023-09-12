use lazy_static::lazy_static;
use regex::Regex;

use crate::slugify;

lazy_static! {
    static ref HEADER_REGEX: Regex = Regex::new(r"<h([1-6])>(.*?)</h[1-6]>").unwrap();
}

pub fn markdown_to_html(markdown: &str) -> Result<String, String> {
    let mut html = markdown::to_html_with_options(markdown, &markdown::Options::gfm())?;
    // Don't we all love parsing HTML with Regex :)

    // Add IDs and links to headings
    html = HEADER_REGEX
        .replace_all(&html, |caps: &regex::Captures| {
            let level = caps.get(1).unwrap().as_str();
            let title = caps.get(2).unwrap().as_str();
            let id = slugify(title);
            format!(r##"<h{level} id="{id}">{title}</h{level}>"##)
        })
        .to_string();

    Ok(html)
}
