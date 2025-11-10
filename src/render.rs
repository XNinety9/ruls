use anyhow::Result;
use crate::model::{Options, EntryInfo, EntryKind};
use time::{OffsetDateTime, UtcOffset};
use time::macros::format_description;
use nu_ansi_term::{Style, Color};

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

    let perms = perms_string(e);
    let size = if opts.bytes {
        e.size.to_string()
    } else {
        format_size(e.size)
    };
    let ts = format_ts(e.mtime);

    let filename_color = file_color(e, opts);

    println!("{perms} {size:>8} {ts} {}", filename_color.paint(e.name.to_string_lossy()));
    Ok(())
}

#[cfg(unix)]
fn perms_string(entry: &EntryInfo) -> String {
    let b = entry.permissions & 0o777;

    // Precompute painted tokens as owned Strings.
    let r_flag = Color::Yellow.paint("r").to_string();
    let w_flag = Color::Red.paint("w").to_string();
    let x_flag = Color::Green.paint("x").to_string();
    let dash   = Style::new().dimmed().paint("-").to_string();

    let mut s = String::with_capacity(10);

    let kind_ch = Color::Blue.paint(
            match entry.kind {
            EntryKind::Dir => "d",
            EntryKind::File => "-",
            EntryKind::Symlink => "l",
            _ => "?",
        }
    ).to_string();

    s.push_str(&kind_ch);

    for (r, w, x) in [(0o400, 0o200, 0o100), (0o040, 0o020, 0o010), (0o004, 0o002, 0o001)] {
        s.push_str(if b & r != 0 { &r_flag } else { &dash });
        s.push_str(if b & w != 0 { &w_flag } else { &dash });
        s.push_str(if b & x != 0 { &x_flag } else { &dash });
    }

    s
}



#[cfg(not(unix))]
fn perms_string(_mode: u32) -> String {
    "---------".to_string()
}

fn format_ts(t: std::time::SystemTime) -> String {
    // Get local offset; fall back to UTC if indeterminate
    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);

    // Convert SystemTime → OffsetDateTime and apply local offset
    let odt = OffsetDateTime::from(t).to_offset(offset);

    let fmt = format_description!("[day] [month repr:short] [year repr:last_two] [hour]:[minute]");
    odt.format(&fmt).unwrap_or_else(|_| "0000-00-00 00:00".into())
}

fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["", "k", "M", "G", "T", "E"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{size:.0}{}", UNITS[unit])
    } else {
        format!("{size:.1}{}", UNITS[unit])
    }
}

fn file_color(entry: &EntryInfo, opts: &Options) -> Style {
    if opts.nocolor {
        return Color::White.normal();
    }
    match entry.kind {
        EntryKind::File => {
            if entry.permissions & 0o111 != 0 {
                return Color::Green.normal()
            }
            Color::White.normal()
        }
        EntryKind::Dir => Color::Blue.bold(),
        EntryKind::Symlink => Color::Cyan.normal(),
        EntryKind::Other => Color::White.normal(),
    }
}
