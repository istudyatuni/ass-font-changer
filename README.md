# Change embedded fonts in .ass subtitles

Do you want to watch movie with subtitles, but don't want to download extra fonts and want to just use a font that's already available? Just do:

## Installation

```sh
cargo install --git https://github.com/istudyatuni/ass-style-changer --locked
```

## Usage

```sh
# Fix single file
assfix fix 'subtitle.ass' --font 'Your Font Name'

# Fix all .ass files in directory
assfix fix ./path/to/dir --font 'Your Font Name'

# Fix all .ass files from "dir" and write to "target" directory
assfix fix ./dir --font 'Your Font Name' --target ./target
```

This will change font for all styles in `[V4+ Styles]` table. Backup of original files will be created, e.g. for `subtitle.ass`: `subtitle.ass.bak`

To disable creating backups pass `--no-backup` flag:

```sh
assfix fix 'subtitle.ass' --font 'Your Font Name' --no-backup
```

## Possible future improvements and features

- Use in browser
- Fix font for specific styles
- Fix more: bold, italic, etc
