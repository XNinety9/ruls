use anyhow::Result;
use crate::model::{Options, EntryInfo, EntryKind};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn print_section_header(header: &str, need_blank_before: bool, _opts: &Options) -> Result<()> {
    if need_blank_before {
        println!();
    }
    println!("{header}:");
    Ok(())
}

pub fn print_entries<'a, I>(items: I, opts: &Options) -> Result<()>
where
    I: IntoIterator<Item = &'a EntryInfo>,
{
    for e in items {
        print_entry(e, opts)?;
    }
    Ok(())
}

pub fn print_entry(e: &EntryInfo, opts: &Options) -> Result<()> {
    if !opts.long {
        println!("{}", e.name.to_string_lossy());
        return Ok(());
    }

    let kind_ch = match e.kind {
        EntryKind::Dir => 'd',
        EntryKind::File => '-',
        EntryKind::Symlink => 'l',
        _ => '?',
    };

    let perms = perms_string(e.permissions);
    let size = e.size;
    let ts = format_ts(e.mtime);

    println!("{kind_ch}{perms} {size:>8} {ts} {}", e.name.to_string_lossy());
    Ok(())
}

#[cfg(unix)]
fn perms_string(mode: u32) -> String {
    let b = mode & 0o777;
    let mut s = String::with_capacity(9);
    for (r, w, x) in [(0o400, 0o200, 0o100), (0o040, 0o020, 0o010), (0o004, 0o002, 0o001)] {
        s.push(if b & r != 0 { 'r' } else { '-' });
        s.push(if b & w != 0 { 'w' } else { '-' });
        s.push(if b & x != 0 { 'x' } else { '-' });
    }
    s
}


#[cfg(not(unix))]
fn perms_string(_mode: u32) -> String {
    "---------".to_string()
}

fn format_ts(t: SystemTime) -> String {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => format!("{}", d.as_secs()),
        Err(_) => "0".to_string(),
    }
}
