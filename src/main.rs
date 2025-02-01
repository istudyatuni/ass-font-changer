use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{Parser, ValueHint};

use ass_style_changer::fix_file;

fn main() -> Result<()> {
    let Cli::Fix(FixArgs {
        path,
        font,
        no_backup,
        target,
    }) = Cli::parse();

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
        if path.extension().is_some_and(|ext| ext != "ass") {
            bail!("file is not .ass file");
        }
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
enum Cli {
    Fix(FixArgs),
}

#[derive(Debug, Parser)]
struct FixArgs {
    /// Path to .ass file or directory with .ass files
    #[clap(value_hint = ValueHint::AnyPath)]
    path: PathBuf,
    /// Font name to set
    #[clap(long)]
    font: String,
    /// Do not create backup files
    #[clap(long)]
    no_backup: bool,
    /// Directory where to write converted files. Default is directory with
    /// original files. Implies `--no-backup`
    #[clap(long)]
    target: Option<PathBuf>,
}
