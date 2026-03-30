use std::{os::unix::fs::MetadataExt, path::PathBuf};
use std::fs;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use ansi_term::Style;
use uzers::{get_user_by_uid, User};

use crate::settings::{Settings, SortBy};
use crate::theme::Theme;
use crate::icon::icon_for;


pub struct Entry {
    pub name: String,
    // pub path: PathBuf,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub mode: u32,
    pub owner: String,
}

impl Entry {
    pub fn format_size(&self, theme: Option<&Theme>) -> String {
        if self.is_dir {
            return match theme {
                Some(t) => t.size_none.paint("-").to_string(),
                None => String::from("-"),
            }
        }

        static UNITS: [&str; 7] = ["", "k", "M", "G", "T", "P", "E"];
        let mut unit = 0;
        if self.size < 1000 {
            return format!("{}", self.size)
        }

        let mut size = self.size as f64;
        while size >= 1000.0{
            size /= 1024.0;
            unit += 1;
        }
        match theme {
            Some(t) => {
                let style = match unit {
                    0 => t.size_bytes,
                    1 => t.size_kbytes,
                    2 => t.size_mbytes,
                    3 => t.size_gbytes,
                    4.. => t.size_tbytes,
                };
                style.paint(format!("{:.1}{}", size, UNITS[unit])).to_string()
            }
            None => format!("{:.1}{}", size, UNITS[unit])
        }
    }

    pub fn format_mode(&self, theme: Option<&Theme>) -> String {
        let Some(theme) = theme else {
            return format!("{}{}{}{}{}{}{}{}{}{}",
                if self.is_dir { "d" } else if self.is_symlink { "l" } else { "-" },
                if self.mode & 0o400 != 0 { "r" } else { "-" },
                if self.mode & 0o200 != 0 { "w" } else { "-" },
                if self.mode & 0o100 != 0 { "x" } else { "-" },
                if self.mode & 0o040 != 0 { "r" } else { "-" },
                if self.mode & 0o020 != 0 { "w" } else { "-" },
                if self.mode & 0o010 != 0 { "x" } else { "-" },
                if self.mode & 0o004 != 0 { "r" } else { "-" },
                if self.mode & 0o002 != 0 { "w" } else { "-" },
                if self.mode & 0o001 != 0 { "x" } else { "-" },
            );
        };

        let bit = |mask: u32, ch: &str, style: Style| -> String {
            if self.mode & mask != 0 { style.paint(ch).to_string() }
            else { theme.perm_dash.paint("-").to_string() }
        };

        let type_char = if self.is_dir         { theme.dir.paint("d") }
                        else if self.is_symlink { theme.symlink.paint("l") }
                        else                   { theme.perm_dash.paint("-") };

        format!("{}{}{}{}{}{}{}{}{}{}",
            type_char,
            bit(0o400, "r", theme.perm_r_user),
            bit(0o200, "w", theme.perm_w_user),
            bit(0o100, "x", theme.perm_x_user),
            bit(0o040, "r", theme.perm_r_group),
            bit(0o020, "w", theme.perm_w_group),
            bit(0o010, "x", theme.perm_x_group),
            bit(0o004, "r", theme.perm_r_other),
            bit(0o002, "w", theme.perm_w_other),
            bit(0o001, "x", theme.perm_x_other),
        )
    }


    fn format_time(&self, theme: &Theme) -> String {
        match self.modified {
            Some(t) => {
                let dt: DateTime<Local> = DateTime::from(t);
                theme.date.paint(dt.format("%d %b %H:%M").to_string()).to_string()
            },
            None => String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_entry() -> Entry {
        Entry {
            name: String::from("foo"),
            // path: PathBuf::from("foo"),
            is_dir: false,
            is_symlink: false,
            size: 0,
            modified: Some(SystemTime::now()),
            mode: 0,
        }
    }

    #[test]
    fn test_format_size_10() {
        let entry = Entry { size: 10, ..dummy_entry() };
        assert_eq!(entry.format_size(None), "10");
    }
    #[test]
    fn test_format_size_1k() {
        let entry = Entry { size: 1024, ..dummy_entry() };
        assert_eq!(entry.format_size(None), "1.0k");
    }
    #[test]
    fn test_format_size_1m() {
        let u1024: u64 = 1024;
        let entry = Entry { size: u1024.pow(2), ..dummy_entry() };
        assert_eq!(entry.format_size(None), "1.0M");
    }
    #[test]
    fn test_format_size_1g() {
        let u1024: u64 = 1024;
        let entry = Entry { size: u1024.pow(3), ..dummy_entry() };
        assert_eq!(entry.format_size(None), "1.0G");
    }
    #[test]
    fn test_format_size_random_value() {
        let entry = Entry { size: 152897, ..dummy_entry() };
        assert_eq!(entry.format_size(None), "149.3k");
    }
    #[test]
    fn test_format_size_random_value_2() {
        let entry = Entry { size: 172210176, ..dummy_entry() };
        assert_eq!(entry.format_size(None), "164.2M");
    }
    #[test]
    fn test_format_size_random_value_3() {
        let entry = Entry { size: 42087612416, ..dummy_entry() };
        assert_eq!(entry.format_size(None), "39.2G");
    }
    #[test]
    fn test_format_mode_777() {
        let entry = Entry { mode: 0o777, ..dummy_entry() };
        assert_eq!(entry.format_mode(None), "-rwxrwxrwx");
    }
    #[test]
    fn test_format_mode_421() {
        let entry = Entry { mode: 0o421, ..dummy_entry() };
        assert_eq!(entry.format_mode(None), "-r---w---x");
    }
    #[test]
    fn test_format_mode_525() {
        let entry = Entry { mode: 0o525, ..dummy_entry() };
        assert_eq!(entry.format_mode(None), "-r-x-w-r-x");
    }
}

pub struct DisplayConfig {
    pub max_size_len: usize,
    // pub max_name_len: usize,
    // plus tard : max_user_len, max_group_len...
}

impl DisplayConfig {
    pub fn from_entries(entries: &[(PathBuf, Vec<Entry>)]) -> DisplayConfig {
        let mut max_size = 0;
        // let mut max_name = 0;

        for (_, dir_contents) in entries {
            for entry in dir_contents {
                let size_len = entry.format_size(None).len();
                // let name_len = entry.name.len();
                if size_len > max_size { max_size = size_len; }
                // if name_len > max_name { max_name = name_len; }
            }
        }

        DisplayConfig { max_size_len: max_size }
    }
}


pub fn read_entries(settings: &Settings) -> Vec<(PathBuf, Vec<Entry>)> {
    let mut result: Vec<(PathBuf, Vec<Entry>)> = Vec::new();

    for path in &settings.paths {
        let mut dir_contents: Vec<Entry> = Vec::new();

        for raw in fs::read_dir(path).unwrap() {
            let raw = raw.unwrap();  // raw : DirEntry

            // file_type() lit le type du lien lui-même, sans suivre la cible
            let file_type = raw.file_type().unwrap();
            let is_symlink = file_type.is_symlink();

            // raw.metadata() = symlink_metadata : ne suit pas le lien.
            // fs::metadata() suit le lien et échoue si la cible est cassée.
            let metadata = match raw.metadata() {
                Ok(m) => m,
                Err(_) => continue, // lien cassé ou permission refusée — on ignore
            };

            let user = match get_user_by_uid(metadata.uid()) {
                Some(s) => s.name().to_string_lossy().into_owned(),
                None => String::from("")
            };

            let entry = Entry {
                name: raw.file_name().to_string_lossy().to_string(),
                // path: raw.path(),
                is_dir: metadata.is_dir(),
                is_symlink,
                size: metadata.len(),
                modified: metadata.modified().ok(), // Ok(t) → Some(t), Err → None
                mode: metadata.mode(),
                owner: user
            };

            dir_contents.push(entry);
        }

        result.push((path.clone(), dir_contents));
    }

    result
}

fn should_include(entry: &Entry, settings: &Settings) -> bool {
    // Fichiers cachés
    if !settings.show_hidden && entry.name.starts_with('.') {
        return false;
    }

    // Ajouter les prochains filtres ici :
    // if settings.only_dirs && !entry.is_dir { return false; }
    // if settings.only_files && entry.is_dir { return false; }

    true
}

pub fn filter_entries(entries: Vec<(PathBuf, Vec<Entry>)>, settings: &Settings) -> Vec<(PathBuf, Vec<Entry>)> {
    let mut result = Vec::new();

    for (path, dir_contents) in entries {
        let mut filtered = Vec::new();

        for entry in dir_contents {
            if should_include(&entry, settings) {
                filtered.push(entry);
            }
        }

        result.push((path, filtered));
    }

    result
}

fn compare_entries(a: &Entry, b: &Entry, settings: &Settings) -> std::cmp::Ordering {
    let by_type = b.is_dir.cmp(&a.is_dir);
    let by_field = match settings.sort_by {
        SortBy::Name => a.name.cmp(&b.name),
        SortBy::Size => a.size.cmp(&b.size),
        SortBy::Date => a.modified.cmp(&b.modified),
    };
    by_type.then(by_field)
}

pub fn sort_entries(
    mut entries: Vec<(PathBuf, Vec<Entry>)>,
    settings: &Settings
) -> Vec<(PathBuf, Vec<Entry>)> {
    for (_, dir_contents) in &mut entries {
        dir_contents.sort_by(|a, b| compare_entries(a, b, settings));
    }
    entries
}

fn display_entry(entry: &Entry, settings: &Settings, display_config: &DisplayConfig, theme: &Theme) {
    let name_and_icon = format!("{} {}", icon_for(entry), entry.name);
    let name = 
        if entry.is_dir                  { theme.dir.paint(name_and_icon) }
        else if entry.is_symlink         { theme.symlink.paint(name_and_icon) }
        else if entry.mode & 0o111 != 0  { theme.executable.paint(name_and_icon) }
        else                             { theme.file.paint(name_and_icon) };

    if settings.long_format == true {
        let size_colored = if entry.is_dir {
            theme.size_bytes.paint("-").to_string()
        } else {
            entry.format_size(Some(theme))
        };
        let size_padding = display_config.max_size_len.saturating_sub(entry.format_size(None).len());
        // let name_padding = display_config.max_name_len.saturating_sub(entry.name.len());

        println!("{} {:>size_pad$}{} {} {} {}",
            entry.format_mode(Some(theme)),
            "", 
            size_colored,   // padding + taille colorée
            entry.owner,
            entry.format_time(theme),
            name,           // padding + nom coloré
            size_pad = size_padding
        )
    } else {
        let suffix = if entry.is_dir { "/" } else { "" };
        println!("{}{}", entry.name, suffix);
    }
}

fn display_dir(path: &PathBuf, contents: &Vec<Entry>, show_header: bool, settings: &Settings, display_config: &DisplayConfig, theme: &Theme) {
    if show_header {
        println!("{}:", path.to_string_lossy());
    }
    for entry in contents {
        display_entry(entry, settings, display_config, theme);
    }
}

pub fn display_entries(entries: &[(PathBuf, Vec<Entry>)], settings: &Settings, display_config: &DisplayConfig) {
    let theme = Theme::default();

    // Afficher l'en-tête du répertoire seulement si on en a plusieurs
    let show_header = entries.len() > 1;

    for (path, contents) in entries {
        display_dir(path, contents, show_header, settings, display_config, &theme);
        if show_header {
            println!(); // ligne vide entre les répertoires
        }
    }
}
