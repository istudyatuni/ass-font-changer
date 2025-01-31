# Change embedded fonts in .ass subtitles

Do you want to watch movie with subtitles, but don't want to download extra fonts and want to just use a font that's already available? Just do:

## Installation

```sh
cargo install --git https://github.com/istudyatuni/ass-font-changer --locked
```

## Usage

```sh
# Fix single file
assfix 'subtitle.ass' 'Your Font Name'

# Fix all .ass files in directory
assfix ./path/to/dir 'Your Font Name'

# Fix all .ass files from "dir" and write to "target" directory
assfix ./dir 'Your Font Name' --target ./target
```

This will change font for all styles in `[V4+ Styles]` table. Backup of original files will be created, e.g. for `subtitle.ass`: `subtitle.ass.bak`

To disable creating backups pass `--no-backup` flag:

```sh
assfix 'subtitle.ass' 'Your Font Name' --no-backup
```

## Possible future improvements and features

- Use in browser
- Fix font for specific styles
- Fix more: bold, italic, etc
