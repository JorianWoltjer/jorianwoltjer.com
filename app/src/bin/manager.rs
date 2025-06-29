use std::{
    env,
    fs::File,
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
            let posts = sqlx::query!("SELECT id, title, markdown FROM posts")
                .fetch_all(&db)
                .await
                .expect("Failed to fetch posts");

            let bar = ProgressBar::new(posts.len() as u64)
                .with_message("Rendering posts")
                .with_style(
                    indicatif::ProgressStyle::with_template(
                        "{spinner:.green} {msg} [{bar:.cyan/blue}] ({pos:>3}/{len:3})",
                    )
                    .unwrap(),
                );
            for post in posts {
                bar.println(format!("Rendering {:?}...", post.title));
                let html = markdown_to_html(&post.markdown).unwrap();
                sqlx::query!("UPDATE posts SET html = $1 WHERE id = $2", html, post.id)
                    .execute(&db)
                    .await
                    .expect("Failed to update post HTML");
                bar.inc(1);
            }
            bar.finish_with_message("Done! All posts saved to database.");
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
        } // TODO: export command to write blog folders, posts and images to filesystem
    }
}
