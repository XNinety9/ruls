use crate::entry::{Entry};
use crate::theme::Theme;
use crate::settings::Settings;
use crate::icon::icon_for;

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

/// Affiche une entrée sur une ligne — format long ou format court selon `settings.long_format`.
fn display_entry(entry: &Entry, settings: &Settings, display_config: &DisplayConfig, theme: &Theme) {
    let name_and_icon = format!("{} {}", icon_for(entry), entry.name);
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

    if settings.long_format {
        let parts: String = columns()
            // iterates over collection
            .iter()
            // wraps elements in tuple with index
            .enumerate()
            // tranforms each element in something else, but only when the collection is consumed
            .map(|(i, c)| {
                // Version colorée de la valeur (avec codes ANSI)
                let colored = (c.display)(entry, theme);

                // Longueur visible — sans les codes ANSI, pour calculer le padding correct
                let visible_len = (c.raw)(entry).len();

                // Nombre d'espaces à ajouter pour atteindre la largeur max de la colonne
                let pad = display_config.col_widths[i].saturating_sub(visible_len);

                // On colle la valeur colorée et le padding (espaces après = alignement gauche)
                if c.alignment == Alignment::Left {
                    format!("{}{:>pad$}", colored, "", pad = pad)
                } else {
                    format!("{:<pad$}{}", "", colored,  pad = pad)
                }
            })
            // Rassemble les colonnes dans un Vec<String>
            .collect::<Vec<_>>()
            // Puis les joint avec un espace entre chaque
            .join(" ");

        println!("{} {}", parts, name);
    } else {
        let suffix = if entry.is_dir { "/" } else { "" };
        println!("{}{}", entry.name, suffix);
    }
}

/// Affiche le contenu d'un répertoire, avec optionnellement son chemin en en-tête
/// et la ligne de titres des colonnes si `settings.show_header` est actif.
fn display_dir(path: &PathBuf, contents: &Vec<Entry>, show_header: bool, settings: &Settings, display_config: &DisplayConfig, theme: &Theme) {
    if show_header {
        println!("{}:", path.to_string_lossy());
    }
    if settings.long_format && settings.show_header {
        let headers: String = columns().iter().enumerate()
            .map(|(i, c)| format!("{:<width$}", c.header, width = display_config.col_widths[i]))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{} {}", theme.header.paint(headers), theme.header.paint("Name        "));
    }
    for entry in contents {
        display_entry(entry, settings, display_config, theme);
    }
}

/// Définit une colonne d'affichage en mode long.
/// `display` produit la version colorée, `raw` la version brute pour calculer les largeurs.
struct Column {
    header: &'static str,
    alignment: Alignment,
    display: fn(&Entry, &Theme) -> String,
    raw:     fn(&Entry)         -> String,
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
        Column { header: "Perms",  alignment: Alignment::Left, display: |e, t| e.format_mode(Some(t)), raw: |e| e.format_mode(None) },
        Column { header: "Size",   alignment: Alignment::Right, display: |e, t| e.format_size(Some(t)), raw: |e| e.format_size(None) },
        Column { header: "Owner",  alignment: Alignment::Right, display: |e, _| e.owner.clone(),         raw: |e| e.owner.clone()     },
        Column { header: "Date",   alignment: Alignment::Left, display: |e, t| e.format_time(Some(t)), raw: |e| e.format_time(None)  },
    ]
}

/// Largeurs maximales calculées sur l'ensemble des entrées, pour aligner les colonnes.
pub struct DisplayConfig {
    pub col_widths:   Vec<usize>,
}

impl DisplayConfig {
    /// Parcourt toutes les entrées pour calculer la largeur maximale de chaque colonne.
    pub fn from_entries(entries: &[(PathBuf, Vec<Entry>)]) -> DisplayConfig {
        let cols = columns();
        let mut col_widths: Vec<usize> = cols.iter().map(|c| c.header.len()).collect();

        for (_, dir_contents) in entries {
            for entry in dir_contents {
                for (i, col) in cols.iter().enumerate() {
                    let w = (col.raw)(entry).len();
                    if w > col_widths[i] { col_widths[i] = w; }
                }
            }
        }

        DisplayConfig { col_widths }
    }
}
