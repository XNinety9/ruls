use anyhow::Result;
use crate::{model::Options, scan, describe, order, render};
use std::{env, path::PathBuf};

pub fn run(opts: &Options, operands: &[PathBuf]) -> Result<()> {
    // compute effective operand list
    let mut paths: Vec<PathBuf> = if operands.is_empty() {
        vec![env::current_dir()?]
    } else {
        operands.to_vec()
    };

    let multi = paths.len() > 1;
    let mut need_blank = false;

    for path in paths.drain(..) {
        if !scan::is_dir(opts, &path)? {
            let entry = describe::describe_entry(opts, &path)?;
            render::print_entry(&entry, opts)?;
            continue;
        }

        if multi {
            render::print_section_header(&path.display().to_string(), need_blank, opts)?;
            need_blank = true;
        }

        let child_paths = scan::read_dir_paths(opts, &path)?;
        let mut entries = Vec::with_capacity(child_paths.len());
        for p in child_paths {
            let info = describe::describe_entry(opts, &p)?;
            entries.push(info);
        }

        if !opts.show_hidden {
            entries.retain(|e| !e.name.to_string_lossy().starts_with('.'));
        }

        order::sort_entries(&mut entries, opts)?;
        render::print_entries(&entries, opts)?;
    }

    Ok(())
}
