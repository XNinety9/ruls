use std::path::PathBuf;

pub struct Settings {
    pub paths: Vec<PathBuf>,
    pub show_hidden: bool,      // -a
    pub long_format: bool,      // -l
    pub sort_by: SortBy,     // --sort
}

pub enum SortBy { Name}

impl Settings {
    pub fn from_args(args: &[String]) -> Settings {
        let mut settings = Settings::default();

        for arg in args.iter().skip(1) {
            if arg.starts_with("--") {
                // Parse long flags
            } else if arg.starts_with("-") {
                // Parse short flags
                let arg = &arg[1..];
                match arg {
                    "a" => settings.show_hidden = true,
                    "s" => settings.sort_by = SortBy::Name,
                    "l" => settings.long_format = true,
                    _ => eprintln!("Unknown parameter '-{}', ignoring", arg)
                }
            } else {
                // These are the paths to explore
                settings.paths.push(PathBuf::from(arg));
            }
        }

        // If no directory to explore was provided, list current directory
        if settings.paths.is_empty() {
            settings.paths.push(PathBuf::from("."));
        }

        settings
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            paths: Vec::new(),
            show_hidden: false,
            sort_by: SortBy::Name,
            long_format: false,
        }
    }
}