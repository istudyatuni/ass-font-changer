use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use aspasia::{AssSubtitle, Subtitle};
use clap::{Parser, ValueHint};

fn main() -> Result<()> {
    let Cli {
        path,
        font,
        no_backup,
    } = Cli::parse();

    let paths = if path.is_dir() {
        glob::glob("./*.ass")
            .context("failed to find .ass files in current directory")?
            .map(|res| res.map_err(Into::into))
            .collect::<Result<_>>()?
    } else {
        vec![path]
    };

    for p in paths {
        if !no_backup {
            let bak = p.with_extension("ass.bak");
            if !bak.exists() {
                std::fs::copy(&p, bak).context("failed to backup file")?;
            }
        }
        fix_file(&p, &font)?;
    }

    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    /// Path to .ass file or directory with .ass files
    #[clap(value_hint = ValueHint::AnyPath)]
    path: PathBuf,
    /// Font name to set
    font: String,
    /// Do not create backup files
    #[clap(long)]
    no_backup: bool,
}

fn fix_file(path: &Path, font: &str) -> Result<()> {
    let mut ass = AssSubtitle::from_path(path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    ass.styles_mut()
        .iter_mut()
        .for_each(|s| s.fontname = font.to_string());

    ass.export(path)?;

    Ok(())
}
