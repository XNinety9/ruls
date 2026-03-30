use std::{env};

mod entry;
mod settings;
mod theme;
mod icon;

use settings::Settings;
use entry::{read_entries, filter_entries, sort_entries, display_entries, DisplayConfig};

/// Point d'entrée : parse les arguments, lit, filtre, trie puis affiche les entrées.
fn main() {
    let args: Vec<String> = env::args().collect();

    let settings = Settings::from_args(&args);
    let entries = read_entries(&settings);
    let entries = filter_entries(entries, &settings);
    let entries = sort_entries(entries, &settings);
    let display_config = DisplayConfig::from_entries(&entries);
    display_entries(&entries, &settings, &display_config);
}
