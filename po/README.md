# Translation Files

This directory contains translation files for the Lorem Ipsum Generator application.

## Files

- `loremgenerator.pot` - Translation template with all translatable strings
- `es.po` - Spanish translation
- `de.po` - German translation
- `fr.po` - French translation
- `pt_BR.po` - Portuguese (Brazilian) translation
- `it.po` - Italian translation
- `ru.po` - Russian translation
- `ja.po` - Japanese translation

Compiled translations (.mo files) are generated during build and placed in:
```
target/debug/locale/<lang>/LC_MESSAGES/loremgenerator.mo
target/release/locale/<lang>/LC_MESSAGES/loremgenerator.mo
```

## Adding a New Translation

1. Copy the template file:
   ```bash
   cp loremgenerator.pot <language_code>.po
   ```

2. Edit the header information in the new file:
   - Set `Language` to your language code (e.g., `fr`, `de`, `pt_BR`)
   - Update translator information

3. Translate all `msgstr` entries

4. Test by building the application:
   ```bash
   cargo build
   ```

## Updating Translations

When new translatable strings are added to the source code:

1. Extract new strings and update the template:
   ```bash
   xgettext --from-code=UTF-8 -k -ktr! -o po/loremgenerator.pot \
     src/*.rs src/helpers/*.rs
   ```

2. Merge updates into existing translations:
   ```bash
   msgmerge -U po/es.po po/loremgenerator.pot
   ```

3. Edit the .po file to translate any new strings

## Testing Translations

To test translations:
```bash
LANGUAGE=es ./target/debug/loremgenerator     # Spanish
LANGUAGE=de ./target/debug/loremgenerator     # German
LANGUAGE=fr ./target/debug/loremgenerator     # French
LANGUAGE=pt_BR ./target/debug/loremgenerator  # Portuguese (Brazilian)
LANGUAGE=it ./target/debug/loremgenerator     # Italian
LANGUAGE=ru ./target/debug/loremgenerator     # Russian
LANGUAGE=ja ./target/debug/loremgenerator     # Japanese
```

Or use the interactive convenience script:
```bash
./run_localized.sh
```

## Requirements

The build system requires the `msgfmt` tool from gettext:

- Ubuntu/Debian: `sudo apt install gettext`
- Fedora: `sudo dnf install gettext`
- Arch: `sudo pacman -S gettext`
