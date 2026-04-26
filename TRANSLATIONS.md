# Translation Support

Lorem Ipsum Generator now supports multiple languages through gettext-based internationalization (i18n).

## Running with a Specific Language

### Method 1: Using environment variables

```bash
LANGUAGE=es ./target/debug/loremgenerator     # Spanish
LANGUAGE=de ./target/debug/loremgenerator     # German
#and so on...
```

### Method 2: Using the provided script

A convenience script is provided to run the application with language selection:

```bash
./run_localized.sh
```

The script will prompt you to select your preferred language.

## Adding a New Translation

1. Copy the translation template:
   ```bash
   cp po/loremgenerator.pot po/<language_code>.po
   ```

2. Edit the header in the new `.po` file:
   - Set the `Language` field (e.g., `fr`, `de`, `pt_BR`)
   - Update translator information
   - Set the proper `Plural-Forms` for your language

3. Translate all `msgstr` entries that currently have empty strings

4. Build the application to compile the translation:
   ```bash
   cargo build
   ```

5. Test your translation:
   ```bash
   LANGUAGE=<language_code> ./target/debug/loremgenerator
   ```

## Translation File Structure

```
po/
├── README.md               # Translation documentation
├── loremgenerator.pot      # Translation template (all extractable strings)
├── es.po                   # Spanish translation
└── <lang>.po              # Other language translations
```

After building, compiled translations are placed in:
```
target/debug/locale/<lang>/LC_MESSAGES/loremgenerator.mo
target/release/locale/<lang>/LC_MESSAGES/loremgenerator.mo
```

## Requirements

### Build-time Requirements

The `msgfmt` tool from gettext is required to compile translations:

- **Ubuntu/Debian**: `sudo apt install gettext`
- **Fedora**: `sudo dnf install gettext`
- **Arch Linux**: `sudo pacman -S gettext`

### Runtime Requirements

The application uses the system's gettext library, which is typically pre-installed on Linux systems.

## For Developers

### Marking Strings for Translation

In Rust code, wrap user-facing strings with the `tr!` macro:

```rust
// Instead of:
set_label: "Generate"

// Use:
set_label: &tr!("Generate")
```

For strings used outside the macro context:

```rust
use gettextrs::gettext;

let translated = gettext("My translatable string");
```

### Testing

To verify translations are working:

1. Build the project: `cargo build`
2. Check compiled translations: `ls target/debug/locale/*/LC_MESSAGES/`
3. Run with specific language: `LANGUAGE=es ./target/debug/loremgenerator`

## Troubleshooting

### Translations not appearing

1. **Check locale is installed on your system:**
   ```bash
   locale -a | grep es
   ```
   If not listed, install it: `sudo locale-gen es_ES.UTF-8`

2. **Verify .mo file exists:**
   ```bash
   ls target/debug/locale/es/LC_MESSAGES/loremgenerator.mo
   ```

3. **Check environment variables:**
   ```bash
   echo $LANGUAGE $LC_ALL $LANG
   ```

4. **Test with verbose output:**
   ```bash
   LANGUAGE=es RUST_LOG=debug ./target/debug/loremgenerator
   ```

### Build warnings about msgfmt

If you see warnings during build that `msgfmt` is not found:
- Install gettext tools (see Requirements section above)
- The application will still build and run, but translations won't be available

## Contributing Translations

Contributions of new translations are welcome! Please:

1. Create a new `.po` file for your language
2. Translate all strings
3. Test the translation thoroughly
4. Submit a pull request with:
   - The new `.po` file
   - Updated documentation listing the new language
