use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use app::{
    cli::{Args, Commands, ThemeCommand},
    render::markdown_to_html,
};
use clap::Parser;
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use syntect::{highlighting::ThemeSet, html::css_for_theme_with_class_style};

lazy_static! {
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
    static ref TARGET_PATH: PathBuf = {
        let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        cargo_dir.join("static/assets/css/theme.css")
    };
}

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
                    let target_path = TARGET_PATH.as_path();
                    let diff_path =
                        pathdiff::diff_paths(target_path, env::current_dir().unwrap()).unwrap();
                    println!("Writing theme {name:?} to {diff_path:?}...");

                    let file = File::create(target_path).unwrap();
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
    }
}
