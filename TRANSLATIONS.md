# Translation Support

Lorem Ipsum Generator now supports multiple languages through gettext-based internationalization (i18n).

## Available Languages

- **English** (en - default)
- **Spanish** (es)
- **German** (de)
- **French** (fr)
- **Portuguese (Brazilian)** (pt_BR)
- **Italian** (it)
- **Russian** (ru)
- **Japanese** (ja)

## Running with a Specific Language

### Method 1: Using environment variables

```bash
LANGUAGE=es ./target/debug/loremgenerator     # Spanish
LANGUAGE=de ./target/debug/loremgenerator     # German
LANGUAGE=fr ./target/debug/loremgenerator     # French
LANGUAGE=pt_BR ./target/debug/loremgenerator  # Portuguese (Brazilian)
LANGUAGE=it ./target/debug/loremgenerator     # Italian
LANGUAGE=ru ./target/debug/loremgenerator     # Russian
LANGUAGE=ja ./target/debug/loremgenerator     # Japanese
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

## Updating Translations

When new translatable strings are added to the source code:

1. Run the update script:
   ```bash
   ./update_translations.sh
   ```

2. Edit the updated `.po` files to translate any new or modified strings

3. Rebuild the application:
   ```bash
   cargo build
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
- **macOS**: `brew install gettext`

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

### Extracting Strings

The `update_translations.sh` script uses `xgettext` to extract strings marked with:
- `tr!` macro
- `gettext()` function

### Testing

To verify translations are working:

1. Build the project: `cargo build`
2. Check compiled translations: `ls target/debug/locale/*/LC_MESSAGES/`
3. Run with specific language: `LANGUAGE=es ./target/debug/loremgenerator`

## Translation Details

### Spanish Translation

All user-facing strings have been translated to Spanish:

| English | Spanish |
|---------|---------|
| _About | _Acerca de |
| Max Words | Máximo de Palabras |
| Max Sentences | Máximo de Oraciones |
| Paragraphs | Párrafos |
| Start with 'Lorem ipsum' | Comenzar con 'Lorem ipsum' |
| _Generate | _Generar |
| _Copy to Clipboard | _Copiar al Portapapeles |
| Copied to clipboard | Copiado al portapapeles |
| Lorem Ipsum Generator | Generador de Lorem Ipsum |

### German Translation

All user-facing strings have been translated to German:

| English | German |
|---------|--------|
| _About | Über |
| Max Words | Maximale Wörter |
| Max Sentences | Maximale Sätze |
| Paragraphs | Absätze |
| Start with 'Lorem ipsum' | Mit 'Lorem ipsum' beginnen |
| _Generate | _Generieren |
| _Copy to Clipboard | In _Zwischenablage kopieren |
| Copied to clipboard | In Zwischenablage kopiert |
| Lorem Ipsum Generator | Lorem Ipsum Generator |

### French Translation

All user-facing strings have been translated to French:

| English | French |
|---------|--------|
| _About | À _propos |
| Max Words | Mots maximum |
| Max Sentences | Phrases maximum |
| Paragraphs | Paragraphes |
| Start with 'Lorem ipsum' | Commencer avec 'Lorem ipsum' |
| _Generate | _Générer |
| _Copy to Clipboard | _Copier dans le presse-papiers |
| Copied to clipboard | Copié dans le presse-papiers |
| Lorem Ipsum Generator | Générateur Lorem Ipsum |

### Portuguese (Brazilian) Translation

All user-facing strings have been translated to Portuguese:

| English | Portuguese (Brazilian) |
|---------|------------------------|
| _About | _Sobre |
| Max Words | Máximo de Palavras |
| Max Sentences | Máximo de Frases |
| Paragraphs | Parágrafos |
| Start with 'Lorem ipsum' | Começar com 'Lorem ipsum' |
| _Generate | _Gerar |
| _Copy to Clipboard | _Copiar para a Área de Transferência |
| Copied to clipboard | Copiado para a área de transferência |
| Lorem Ipsum Generator | Gerador de Lorem Ipsum |

### Italian Translation

All user-facing strings have been translated to Italian:

| English | Italian |
|---------|---------|
| _About | _Informazioni |
| Max Words | Parole massime |
| Max Sentences | Frasi massime |
| Paragraphs | Paragrafi |
| Start with 'Lorem ipsum' | Inizia con 'Lorem ipsum' |
| _Generate | _Genera |
| _Copy to Clipboard | _Copia negli appunti |
| Copied to clipboard | Copiato negli appunti |
| Lorem Ipsum Generator | Generatore di Lorem Ipsum |

### Russian Translation

All user-facing strings have been translated to Russian:

| English | Russian |
|---------|---------|
| _About | _О программе |
| Max Words | Максимум слов |
| Max Sentences | Максимум предложений |
| Paragraphs | Параграфы |
| Start with 'Lorem ipsum' | Начать с 'Lorem ipsum' |
| _Generate | _Генерировать |
| _Copy to Clipboard | _Копировать в буфер обмена |
| Copied to clipboard | Скопировано в буфер обмена |
| Lorem Ipsum Generator | Генератор Lorem Ipsum |

### Japanese Translation

All user-facing strings have been translated to Japanese:

| English | Japanese |
|---------|----------|
| _About | このアプリケーションについて(_A) |
| Max Words | 最大単語数 |
| Max Sentences | 最大文数 |
| Paragraphs | 段落 |
| Start with 'Lorem ipsum' | 'Lorem ipsum'で始める |
| _Generate | 生成(_G) |
| _Copy to Clipboard | クリップボードにコピー(_C) |
| Copied to clipboard | クリップボードにコピーしました |
| Lorem Ipsum Generator | Lorem Ipsum ジェネレーター |

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
