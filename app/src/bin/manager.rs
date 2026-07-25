use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use app::{
    cli::{Args, Commands, ThemeCommand},
    render::markdown_to_html,
};
use clap::Parser;
use fancy_regex::{Captures, Regex};
use indicatif::ProgressBar;
use sqlx::postgres::PgPoolOptions;
use syntect::{highlighting::ThemeSet, html::css_for_theme_with_class_style};
use tokio::task;

static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);
static BASE_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf());
static FONT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?P<before_url>/\*\s*(?P<variant>[\w-]+)\s*\*/\s*@font-face\s*\{\s*font-family:\s*'(?P<name>[^']+)';\s*font-style:\s*(?P<style>\w+);\s*[^}]*?src:\s*url\()(?P<url>[^)]+)(?P<after_url>\)[^}]*\})"#).unwrap()
});

#[tokio::main]
async fn main() {
    let args = Args::parse();

    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    match args.command {
        Commands::Theme { command } => match command {
            ThemeCommand::List => {
                println!("Available themes:");
                for theme in THEME_SET.themes.keys() {
                    println!("- {theme:?}");
                }
            }
            ThemeCommand::Set { name } => {
                if let Some(theme) = THEME_SET.themes.get(&name) {
                    let theme_path = BASE_DIR.join("static/assets/css/theme.css");
                    let diff_path =
                        pathdiff::diff_paths(&theme_path, env::current_dir().unwrap()).unwrap();
                    println!("Writing theme {name:?} to {diff_path:?}...");

                    let file = File::create(theme_path).unwrap();
                    let mut writer = BufWriter::new(&file);
                    let css = css_for_theme_with_class_style(
                        theme,
                        syntect::html::ClassStyle::SpacedPrefixed { prefix: "hl-" },
                    )
                    .unwrap();
                    writer.write_all(css.as_bytes()).unwrap();
                    println!("Written {} bytes to successfully", css.len());
                } else {
                    println!("Theme {name:?} not found.");
                }
            }
        },
        Commands::Render => {
            // Re-render all posts in database
            let posts = sqlx::query!("SELECT id, title, markdown, html FROM posts")
                .fetch_all(&db)
                .await
                .expect("Failed to fetch posts");

            let total = posts.len();
            let bar = ProgressBar::new(total as u64)
                .with_message("Rendering posts")
                .with_style(
                    indicatif::ProgressStyle::with_template(
                        "{spinner:.green} {msg} [{bar:.cyan/blue}] ({pos:>3}/{len:3})",
                    )
                    .unwrap(),
                );
            let mut changed = 0usize;
            for post in posts {
                let html = markdown_to_html(&post.markdown).unwrap();
                if html != post.html {
                    bar.println(format!("Updating {:?}...", post.title));
                    sqlx::query!("UPDATE posts SET html = $1 WHERE id = $2", html, post.id)
                        .execute(&db)
                        .await
                        .expect("Failed to update post HTML");
                    changed += 1;
                }
                bar.inc(1);
            }
            bar.finish_with_message(format!(
                "Done! {changed} changed, {} unchanged.",
                total - changed
            ));
        }
        Commands::Password => {
            // Set administrator password
            let password = rpassword::prompt_password("New password: ").unwrap();
            if password.is_empty() {
                println!("Password cannot be empty.");
                return;
            }
            let hashed_password = bcrypt::hash(password, 12).unwrap();
            sqlx::query!(
                "INSERT INTO secrets (name, value) VALUES ('password_hash', $1)
                 ON CONFLICT (name) DO UPDATE SET value = $1",
                hashed_password
            )
            .execute(&db)
            .await
            .expect("Failed to set password");
            println!("Administrator password set successfully.");
        }
        Commands::Fonts { url } => {
            // Download CSS & fonts locally from Google Fonts
            println!("Downloading CSS...");
            let client = reqwest::Client::new();
            let mut css = client
                .get(&url)
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0")
                .send()
                .await
                .expect("Failed to download CSS")
                .text()
                .await
                .expect("Failed to read CSS text");
            css = task::spawn_blocking(move || {
                FONT_REGEX
                    .replace_all(&css, |caps: &Captures| {
                        let before_url = caps.name("before_url").unwrap().as_str();
                        let variant = caps.name("variant").unwrap().as_str();
                        let name = caps.name("name").unwrap().as_str();
                        let style = caps.name("style").unwrap().as_str();
                        let url = caps.name("url").unwrap().as_str();
                        let after_url = caps.name("after_url").unwrap().as_str();

                        // Download the font file
                        let font_name = format!(
                            "{name}_{variant}{}.woff2",
                            if style != "normal" {
                                format!("_{style}")
                            } else {
                                String::new()
                            }
                        );
                        let font_path = BASE_DIR.join(format!("static/assets/fonts/{font_name}"));
                        let mut font_file = File::create(&font_path).unwrap();
                        println!("Downloading {font_name:?} from {url}...");
                        let font_data = reqwest::blocking::get(url)
                            .expect("Failed to download font")
                            .bytes()
                            .expect("Failed to read font bytes");
                        font_file.write_all(&font_data).unwrap();

                        // Return the updated CSS rule
                        format!("{before_url}'/assets/fonts/{font_name}'{after_url}")
                    })
                    .to_string()
            })
            .await
            .unwrap();
            let css_path = BASE_DIR.join("static/assets/css/fonts.css");
            let mut css_file = File::create(css_path).expect("Failed to create CSS file");
            css_file
                .write_all(css.as_bytes())
                .expect("Failed to write CSS file");

            println!("Fonts downloaded and CSS updated successfully.");
        }
        Commands::Seed => {
            seed_blog(&db).await;
        }
        Commands::Export { directory } => {
            export_blog(&db, &directory).await;
        }
    }
}

async fn export_blog(db: &sqlx::PgPool, directory: &Path) {
    fs::create_dir_all(directory).expect("Failed to create export directory");

    // Copy images
    let src_img = BASE_DIR.join("static/img/blog");
    let dest_img = directory.join("img/blog");
    fs::create_dir_all(&dest_img).expect("Failed to create img/blog directory");
    let mut image_count = 0usize;
    for entry in fs::read_dir(&src_img).expect("Failed to read static/img/blog") {
        let entry = entry.expect("Failed to read image directory entry");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = entry.file_name();
        if name == ".gitignore" {
            continue;
        }
        fs::copy(&path, dest_img.join(&name)).expect("Failed to copy image");
        image_count += 1;
    }
    println!("Copied {image_count} images to img/blog/");

    // Export folders as README.md
    let folders = sqlx::query!("SELECT slug, title, description FROM folders")
        .fetch_all(db)
        .await
        .expect("Failed to fetch folders");
    for folder in &folders {
        let folder_dir = directory.join(&folder.slug);
        fs::create_dir_all(&folder_dir).expect("Failed to create folder directory");
        let readme = format!("# {}\n\n{}\n", folder.title, folder.description);
        fs::write(folder_dir.join("README.md"), readme).expect("Failed to write folder README");
    }
    println!("Wrote {} folder README.md files", folders.len());

    // Export posts as {slug}.md
    let posts = sqlx::query!("SELECT slug, title, markdown FROM posts")
        .fetch_all(db)
        .await
        .expect("Failed to fetch posts");
    let bar = ProgressBar::new(posts.len() as u64)
        .with_message("Exporting posts")
        .with_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} {msg} [{bar:.cyan/blue}] ({pos:>3}/{len:3})",
            )
            .unwrap(),
        );
    for post in &posts {
        let post_path = directory.join(format!("{}.md", post.slug));
        if let Some(parent) = post_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create post directory");
        }
        let content = format!("# {}\n\n{}", post.title, post.markdown);
        fs::write(&post_path, content).expect("Failed to write post markdown");
        bar.inc(1);
    }
    bar.finish_with_message(format!("Exported {} posts", posts.len()));
}

const SEED_MARKDOWN: &str = r#"This post is inserted by `manager seed` so local development and render/export testing have something to look at.

## Formatting

- **Bold**, *italic*, and `inline code`
- A [external link](https://example.com)
- Nested list:
  1. First
  2. Second

## Code

```rust
fn main() {
    println!("Hello, blog!");
}
```

```python+wrap
print("long line that may wrap when the +wrap language suffix is used in fenced code blocks")
```

## Media

Placeholder image:

![Placeholder cover](placeholder.png)

<iframe src="https://youtube-nocookie.com/embed/dQw4w9WgXcQ"></iframe>

## Heading with HTML entities & punctuation!

Text under a heading that needs slugification.
"#;

async fn seed_blog(db: &sqlx::PgPool) {
    let existing =
        sqlx::query_scalar!("SELECT id FROM posts WHERE slug = 'ctf/sample-ctf/hello-world'")
            .fetch_optional(db)
            .await
            .expect("Failed to check for existing seed post");

    if existing.is_some() {
        println!("Seed data already present (ctf/sample-ctf/hello-world); nothing to do.");
        return;
    }

    let ctf_id = sqlx::query_scalar!("SELECT id FROM folders WHERE slug = 'ctf'")
        .fetch_optional(db)
        .await
        .expect("Failed to look up ctf folder")
        .expect("Root folder 'ctf' missing — run migrations first");

    let event_id = sqlx::query_scalar!(
        "INSERT INTO folders (parent, slug, title, description, img)
         VALUES ($1, 'ctf/sample-ctf', 'Sample CTF', 'A nested folder created by manager seed for testing.', 'fa-solid fa-flag')
         RETURNING id",
        ctf_id
    )
    .fetch_one(db)
    .await
    .expect("Failed to create sample folder");

    let html = markdown_to_html(SEED_MARKDOWN).expect("Failed to render seed markdown");

    let post_id = sqlx::query_scalar!(
        "INSERT INTO posts (folder, title, slug, description, img, points, featured, hidden, markdown, html)
         VALUES ($1, 'Hello World', 'ctf/sample-ctf/hello-world',
                 'Sample writeup generated by manager seed.',
                 'placeholder.png', 100, true, false, $2, $3)
         RETURNING id",
        event_id,
        SEED_MARKDOWN,
        html
    )
    .fetch_one(db)
    .await
    .expect("Failed to create sample post");

    sqlx::query!(
        "INSERT INTO post_tags (post_id, tag_id)
         SELECT $1, id FROM tags WHERE name = ANY(ARRAY['Web', 'Scripting'])",
        post_id
    )
    .execute(db)
    .await
    .expect("Failed to attach tags");

    let hidden_html = markdown_to_html("Draft content only visible via the hidden-post link.\n")
        .expect("Failed to render hidden post markdown");
    sqlx::query!(
        "INSERT INTO posts (folder, title, slug, description, img, points, featured, hidden, markdown, html)
         VALUES ($1, 'Hidden Draft', 'ctf/sample-ctf/hidden-draft',
                 'Hidden post for testing signed URLs.',
                 'placeholder.png', 0, false, true, $2, $3)",
        event_id,
        "Draft content only visible via the hidden-post link.\n",
        hidden_html
    )
    .execute(db)
    .await
    .expect("Failed to create hidden post");

    sqlx::query!(
        "INSERT INTO links (folder, url, title, description, img, featured)
         VALUES ($1, 'https://example.com', 'Example Link', 'A sample link card.', 'placeholder.png', false)",
        event_id
    )
    .execute(db)
    .await
    .expect("Failed to create sample link");

    println!("Seeded blog structure:");
    println!("  /blog/f/ctf/sample-ctf");
    println!("  /blog/p/ctf/sample-ctf/hello-world  (featured)");
    println!("  /blog/h/ctf/sample-ctf/hidden-draft (hidden)");
    println!("  link → https://example.com");
}
