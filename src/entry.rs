use std::{os::unix::fs::MetadataExt, path::PathBuf};
use std::fs;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use ansi_term::Style;
use uzers::get_user_by_uid;

use crate::settings::{Settings, SortBy};
use crate::theme::Theme;

/// Représente un fichier ou répertoire avec toutes ses métadonnées.
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
    /// Formate la taille en unités lisibles (k, M, G…).
    /// Si `theme` est fourni, applique la couleur correspondant à l'ordre de grandeur.
    /// Retourne `"-"` pour les répertoires.
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

    /// Formate les permissions Unix au format `drwxrwxrwx`.
    /// Si `theme` est fourni, chaque bit est coloré individuellement.
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

    /// Formate la date de dernière modification au format `JJ Mmm HH:MM`.
    /// Si `theme` est fourni, applique la couleur de date. Retourne `""` si la date est indisponible.
    pub fn format_time(&self, theme: Option<&Theme>) -> String {
        match self.modified {
            Some(t) => {
                let dt: DateTime<Local> = DateTime::from(t);
                let s = dt.format("%d %b %H:%M").to_string();
                match theme {
                    Some(t) => t.date.paint(s).to_string(),
                    None    => s,
                }
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
            owner: String::new()
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

/// Lit le contenu des répertoires spécifiés dans `settings.paths`.
/// Les liens symboliques cassés ou les entrées inaccessibles sont silencieusement ignorés.
pub fn read_entries(settings: &Settings) -> Vec<(PathBuf, Vec<Entry>)> {
    let mut result: Vec<(PathBuf, Vec<Entry>)> = Vec::new();

    for path in &settings.paths {
        let mut dir_contents: Vec<Entry> = Vec::new();

        let read_dir = match fs::read_dir(path) {
            Ok(r) => r,
            Err(_) => {
                eprintln!("Error: file or folder not found '{}'", path.to_string_lossy());
                continue;
            }
        };

        for raw in read_dir {
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

/// Retourne `true` si l'entrée doit être affichée selon les paramètres courants.
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

/// Filtre les entrées de chaque répertoire selon les paramètres (fichiers cachés, etc.).
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

/// Compare deux entrées : répertoires avant fichiers, puis selon `settings.sort_by`.
fn compare_entries(a: &Entry, b: &Entry, settings: &Settings) -> std::cmp::Ordering {
    let by_type = b.is_dir.cmp(&a.is_dir);
    let by_field = match settings.sort_by {
        SortBy::Name => a.name.cmp(&b.name),
        SortBy::Size => a.size.cmp(&b.size),
        SortBy::Date => a.modified.cmp(&b.modified),
    };
    by_type.then(by_field)
}

/// Trie les entrées de chaque répertoire selon les paramètres.
pub fn sort_entries(
    mut entries: Vec<(PathBuf, Vec<Entry>)>,
    settings: &Settings
) -> Vec<(PathBuf, Vec<Entry>)> {
    for (_, dir_contents) in &mut entries {
        dir_contents.sort_by(|a, b| compare_entries(a, b, settings));
    }
    entries
}

