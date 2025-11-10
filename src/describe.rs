use anyhow::{Result, Context};
use crate::model::{EntryInfo, EntryKind, Options};
use std::{fs, path::Path, time::SystemTime};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn describe_entry(_opts: &Options, p: &Path) -> Result<EntryInfo> {
    // do not follow symlinks (closer to `ls`)
    let meta = fs::symlink_metadata(p).with_context(|| format!("stat {}", p.display()))?;
    let ft = meta.file_type();

    let kind = if ft.is_dir() {
        EntryKind::Dir
    } else if ft.is_file() {
        EntryKind::File
    } else if cfg!(unix) && ft.is_symlink() {
        EntryKind::Symlink
    } else {
        EntryKind::Other
    };

    let name = p.file_name().unwrap_or_else(|| p.as_os_str()).to_owned();
    let size = meta.len();
    let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);

    #[cfg(unix)]
    let permissions = meta.permissions().mode();

    #[cfg(not(unix))]
    let permissions = 0;

    Ok(EntryInfo {
        name,
        kind,
        size,
        mtime,
        permissions: permissions as u32,
    })
}
