use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "manager",
    about = "Perform simple management tasks on the website"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Syntax highlighing themes
    Theme {
        #[command(subcommand)]
        command: ThemeCommand,
    },
    /// Update generated HTML for all posts
    HTML,
}

#[derive(Parser, Debug)]
pub enum ThemeCommand {
    /// List all themes
    List,
    /// Select a theme
    Set {
        /// Name of the theme to set
        name: String,
    },
}
