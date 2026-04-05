use std::{env};

mod entry;
mod settings;
mod theme;
mod icon;
mod display;

use settings::Settings;
use entry::{read_entries, filter_entries, sort_entries};
use display::{display_entries, DisplayConfig};

fn main() {
    // Idées
    // TODO: output as JSON/YAML, maybe colored?

    let settings = Settings::from_args(env::args());
    let entries = read_entries(&settings);
    let entries = filter_entries(entries, &settings);
    let entries = sort_entries(entries, &settings);
    let display_config = DisplayConfig::from_entries(&entries);
    display_entries(&entries, &settings, &display_config);
}
