use anyhow::{Result, Context};
use crate::model::Options;
use std::{fs, path::Path, path::PathBuf};

// The "Result<>" here is from anyhow
pub fn is_dir(_opts: &Options, p: &Path) -> Result<bool> {
    // with_context is a method from anyhow that adds context to an error handling
    let meta = fs::metadata(p).with_context(|| format!("checking {}", p.display()))?;
    Ok(meta.is_dir())
}

pub fn read_dir_paths(_opts: &Options, dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for ent in fs::read_dir(dir).with_context(|| format!("reading dir {}", dir.display()))? {
        out.push(ent?.path());
    }
    Ok(out)
}
