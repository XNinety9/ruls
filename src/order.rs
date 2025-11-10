use anyhow::Result;
use crate::model::{Options, EntryInfo};

pub fn sort_entries(entries: &mut [EntryInfo], opts: &Options) -> Result<()> {
    entries.sort_by(|a, b| a.name.to_string_lossy().cmp(&b.name.to_string_lossy()));
    if opts.reverse {
        entries.reverse();
    }
    Ok(())
}
