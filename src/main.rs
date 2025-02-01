use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use aspasia::{AssSubtitle, Subtitle};
use clap::{Parser, ValueHint};

fn main() -> Result<()> {
    let Cli {
        path,
        font,
        no_backup,
        target,
    } = Cli::parse();

    let cur_dir = std::env::current_dir().context("failed to get current directory")?;
    let target = target.map(|t| cur_dir.join(t));

    // do not backup when target specified
    let no_backup = target.is_some() || no_backup;

    if let Some(t) = &target {
        if !t.exists() {
            std::fs::create_dir_all(t).context("failed to create target directory")?;
        }
    }

    let paths = if path.is_dir() {
        glob::glob("./*.ass")
            .context("failed to find .ass files in current directory")?
            .map(|res| res.map_err(Into::into))
            .collect::<Result<_>>()?
    } else {
        vec![path]
    };

    let export_paths = if let Some(t) = target {
        paths
            .iter()
            .map(|p| t.join(p.file_name().expect("should have filename")))
            .collect::<Vec<_>>()
    } else {
        paths.clone()
    };

    for (from, to) in paths.into_iter().zip(export_paths) {
        if !no_backup {
            let bak = from.with_extension("ass.bak");
            if !bak.exists() {
                std::fs::copy(&from, bak).context("failed to backup file")?;
            }
        }
        fix_file(&from, &to, &font)?;
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
    /// Directory where to write converted files. Default is directory with
    /// original files. Implies `--no-backup`
    #[clap(long)]
    target: Option<PathBuf>,
}

fn fix_file(from: &Path, to: &Path, font: &str) -> Result<()> {
    let mut ass = AssSubtitle::from_path(from)
        .with_context(|| format!("failed to read {}", from.display()))?;

    ass.styles_mut()
        .iter_mut()
        .for_each(|s| s.fontname = font.to_string());

    ass.export(to)?;

    Ok(())
}
