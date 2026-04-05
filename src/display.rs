use crate::entry::{Entry};
use crate::theme::Theme;
use crate::settings::Settings;
use crate::icon::icon_for;

use terminal_size::terminal_size;

use std::path::PathBuf;

/// Point d'entrée de l'affichage. Crée le thème et itère sur les répertoires.
/// Affiche le chemin de chaque répertoire en en-tête si plusieurs sont listés.
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

/// Affiche le contenu d'un répertoire, avec optionnellement son chemin en en-tête
/// et la ligne de titres des colonnes si `settings.show_header` est actif.
fn display_dir(path: &PathBuf, contents: &Vec<Entry>, show_header: bool, settings: &Settings, display_config: &DisplayConfig, theme: &Theme) {
    let columns = columns();
    if show_header {
        println!("{}:", path.to_string_lossy());
    }
    if settings.long_format {
        if settings.show_header {
            let headers: String = columns.iter().enumerate()
                .map(|(i, c)| format!("{:<width$}", c.header, width = display_config.col_widths[i]))
                .collect::<Vec<_>>()
                .join(" ");
            println!("{} {}", theme.header.paint(headers), theme.header.paint("Name        "));
        }
        for entry in contents {
            display_entry(entry, &columns, settings, display_config, theme);
        }
    } else {
        let column_width = if settings.show_icons {
            display_config.max_name_length + 4
        } else {
            display_config.max_name_length + 2
        };
        let n_columns = std::cmp::max(1, display_config.terminal_width / column_width as u16) as usize;
        let n_rows = contents.len().div_ceil(n_columns);

        for row in 0..n_rows {
            for column in 0..n_columns {
                let index = column * n_rows + row;
                if index < contents.len() {
                    display_entry(&contents[index], &columns, settings, display_config, theme);
                }
            }
            println!();
        }
    }
}

/// Affiche une entrée sur une ligne — format long ou format court selon `settings.long_format`.
fn display_entry(entry: &Entry, cols: &[Column], settings: &Settings, display_config: &DisplayConfig, theme: &Theme) {
    if settings.long_format {
        let name_and_icon = if settings.show_icons {
            format!("{} {}", icon_for(entry), entry.name)
        } else {
            format!("{}", entry.name)
        };
        let name = if entry.is_dir {
            theme.dir.paint(name_and_icon).to_string()
        } else if entry.is_symlink {
            let destination = entry.symlink_destination.to_string_lossy();
            let arrow = format!("{name_and_icon} -> {destination}");
            if entry.is_symlink_valid { theme.symlink.paint(arrow).to_string() }
            else                       { theme.bkn_symlink.paint(arrow).to_string() }
        } else if entry.mode & 0o111 != 0 {
            theme.executable.paint(name_and_icon).to_string()
        } else {
            theme.file.paint(name_and_icon).to_string()
        };

        let parts: String = cols.iter().enumerate()
            .map(|(i, c)| {
                // Version colorée de la valeur (avec codes ANSI)
                let colored = (c.display)(entry, theme);

                // Longueur visible via len() — pas d'allocation intermédiaire
                let visible_len = (c.len)(entry);

                // Nombre d'espaces à ajouter pour atteindre la largeur max de la colonne
                let pad = display_config.col_widths[i].saturating_sub(visible_len);

                if c.alignment == Alignment::Left {
                    format!("{}{:>pad$}", colored, "", pad = pad)
                } else {
                    format!("{:<pad$}{}", "", colored, pad = pad)
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        println!("{} {}", parts, name);
    } else {
        let ending = if display_config.add_newline_after_each_entry {"\n"} else {""};
        if settings.show_icons {
            print!("{} {}{}", icon_for(entry), entry.name, ending);
        } else {
            let suffix = if entry.is_dir { "/" } else { "" };
            print!("{}{}{}", entry.name, suffix, ending);
        };

    }
}

/// Définit une colonne d'affichage en mode long.
/// `display` produit la version colorée, `len` retourne la longueur visible (sans ANSI).
struct Column {
    header: &'static str,
    alignment: Alignment,
    display: fn(&Entry, &Theme) -> String,
    len:     fn(&Entry)         -> usize,
}

#[allow(unused)]
#[derive(PartialEq)]
enum Alignment {
    Left, Right,
}

/// Retourne la liste ordonnée des colonnes affichées en mode long.
/// C'est ici qu'on ajoute ou retire une colonne.
fn columns() -> Vec<Column> {
    vec![
        Column { header: "Perms",  alignment: Alignment::Left,  display: |e, t| e.format_mode(Some(t)), len: |e| e.format_mode(None).len() },
        Column { header: "Size",   alignment: Alignment::Right, display: |e, t| e.format_size(Some(t)), len: |e| e.format_size(None).len() },
        Column { header: "Owner",  alignment: Alignment::Right, display: |e, _| e.owner.clone(),         len: |e| e.owner.len()             },
        Column { header: "Date",   alignment: Alignment::Left,  display: |e, t| e.format_time(Some(t)), len: |e| e.format_time(None).len()  },
    ]
}

/// Largeurs maximales calculées sur l'ensemble des entrées, pour aligner les colonnes.
pub struct DisplayConfig {
    pub col_widths:   Vec<usize>,
    pub terminal_width: u16,
    pub max_name_length: usize,
    pub add_newline_after_each_entry: bool,
}

impl DisplayConfig {
    /// Parcourt toutes les entrées pour calculer la largeur maximale de chaque colonne.
    pub fn from_entries(entries: &[(PathBuf, Vec<Entry>)], settings: &Settings) -> DisplayConfig {
        let cols = columns();
        let mut max_name_length = 0;
        let mut col_widths: Vec<usize> = cols.iter().map(|c| c.header.len()).collect();

        for (_, dir_contents) in entries {
            for entry in dir_contents {
                for (i, col) in cols.iter().enumerate() {
                    let w = (col.len)(entry);
                    if w > col_widths[i] { col_widths[i] = w; }
                }
                let name_length = entry.name.len();
                if  name_length > max_name_length {
                    max_name_length = name_length;
                }
            }
        }

        let size = terminal_size();
        let width = match size {
            Some(s) => s.0.0,
            None => 80,
        };

        DisplayConfig {
            col_widths,
            terminal_width: width,
            max_name_length: max_name_length,
            add_newline_after_each_entry: !settings.long_format,
        }
    }
}
