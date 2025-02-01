use std::path::Path;

use anyhow::{Context, Result};
use aspasia::{AssSubtitle, Subtitle};

pub fn fix_file(from: &Path, to: &Path, font: &str) -> Result<()> {
    let mut ass = AssSubtitle::from_path(from)
        .with_context(|| format!("failed to read {}", from.display()))?;

    ass.styles_mut()
        .iter_mut()
        .for_each(|s| s.fontname = font.to_string());

    ass.export(to)?;

    Ok(())
}
