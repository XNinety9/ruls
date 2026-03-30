use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "ruls", about = "A modern ls replacement written in Rust")]
pub struct Settings {
    /// Show hidden files (dot files)
    #[arg(short = 'a', long = "all")]
    pub show_hidden: bool,

    /// Long format (permissions, size, date)
    #[arg(short = 'l', long = "long")]
    pub long_format: bool,

    /// Sort by field: name, size, date
    #[arg(short = 's', long = "sort", value_name = "CRITERIA", default_value = "name")]
    pub sort_by: SortBy,

    /// Directories to list (defaults to current directory)
    pub paths: Vec<PathBuf>,
}

#[derive(ValueEnum, Clone)]
pub enum SortBy {
    Name,
    Size,
    Date,
}

impl Settings {
    pub fn from_args(args: &[String]) -> Settings {
        Settings::parse_from(args)
    }
}
