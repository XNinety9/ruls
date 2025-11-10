use std::{ffi::OsString, time::SystemTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Dir,
    Symlink,
    Other,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub show_hidden: bool,
    pub reverse: bool,
    pub long: bool,
    pub bytes: bool,
    pub nocolor: bool,
    // later: long, group_dirs, color, etc.
}

#[derive(Debug, Clone)]
pub struct EntryInfo {
    pub name: OsString,
    pub kind: EntryKind,
    pub size: u64,
    pub mtime: SystemTime,
    pub permissions: u32,
    // later: perms, symlink target, etc.
}
