#!/bin/bash
# Script to run Lorem Ipsum Generator with language selection

# Point gettext at the development-compiled locale files
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export TEXTDOMAINDIR="$SCRIPT_DIR/target/debug/locale"

echo "Lorem Ipsum Generator - Language Selection"
echo "=========================================="
echo "Available languages:"
echo "   1) English (default)"
echo "   2) Spanish (Español)"
echo "   3) German (Deutsch)"
echo "   4) French (Français)"
echo "   5) Portuguese - Brazil (Português - Brasil)"
echo "   6) Portuguese - Portugal (Português)"
echo "   7) Italian (Italiano)"
echo "   8) Russian (Русский)"
echo "   9) Japanese (日本語)"
echo "  10) Ukrainian (Українська)"
echo "  11) Chinese Simplified (简体中文)"
echo "  12) Chinese Traditional (繁體中文)"
echo "  13) Hindi (हिन्दी)"
echo "  14) Polish (Polski)"
echo "  15) Hungarian (Magyar)"
echo "  16) Greek (Ελληνικά)"
echo "  17) Dutch (Nederlands)"
echo "  18) Serbian (Српски)"
echo "  19) Latvian (Latviešu)"
echo "  20) Slovenian (Slovenščina)"
echo "  21) Hebrew (עברית)"
echo "  22) Arabic (العربية)"
echo "  23) Bengali (বাংলা)"
echo "  24) Korean (한국어)"
echo "  25) Vietnamese (Tiếng Việt)"
echo "  26) Thai (ภาษาไทย)"
echo "  27) Indonesian (Bahasa Indonesia)"
echo "  28) Swedish (Svenska)"
echo "  29) Norwegian (Norsk Bokmål)"
echo "  30) Finnish (Suomi)"
echo "  31) Romanian (Română)"
echo "  32) Czech (Čeština)"
echo "  33) Persian (فارسی)"
echo ""
read -p "Select language [1-33, default: 1]: " choice

case $choice in
    2)
        echo "Starting in Spanish..."
        export LANGUAGE=es
        ;;
    3)
        echo "Starting in German..."
        export LANGUAGE=de
        ;;
    4)
        echo "Starting in French..."
        export LANGUAGE=fr
        ;;
    5)
        echo "Starting in Portuguese (Brazilian)..."
        export LANGUAGE=pt_BR
        ;;
    6)
        echo "Starting in Portuguese (Portugal)..."
        export LANGUAGE=pt
        ;;
    7)
        echo "Starting in Italian..."
        export LANGUAGE=it
        ;;
    8)
        echo "Starting in Russian..."
        export LANGUAGE=ru
        ;;
    9)
        echo "Starting in Japanese..."
        export LANGUAGE=ja
        ;;
    10)
        echo "Starting in Ukrainian..."
        export LANGUAGE=uk
        ;;
    11)
        echo "Starting in Chinese (Simplified)..."
        export LANGUAGE=zh_CN
        ;;
    12)
        echo "Starting in Chinese (Traditional)..."
        export LANGUAGE=zh_TW
        ;;
    13)
        echo "Starting in Hindi..."
        export LANGUAGE=hi
        ;;
    14)
        echo "Starting in Polish..."
        export LANGUAGE=pl
        ;;
    15)
        echo "Starting in Hungarian..."
        export LANGUAGE=hu
        ;;
    16)
        echo "Starting in Greek..."
        export LANGUAGE=el
        ;;
    17)
        echo "Starting in Dutch..."
        export LANGUAGE=nl
        ;;
    18)
        echo "Starting in Serbian..."
        export LANGUAGE=sr
        ;;
    19)
        echo "Starting in Latvian..."
        export LANGUAGE=lv
        ;;
    20)
        echo "Starting in Slovenian..."
        export LANGUAGE=sl
        ;;
    21)
        echo "Starting in Hebrew..."
        export LANGUAGE=he LANG=he_IL.UTF-8
        export LANG=he_IL.UTF-8
        ;;
    22)
        echo "Starting in Arabic..."
        export LANGUAGE=ar
        export LANG=ar_SA.UTF-8
        ;;
    23)
        echo "Starting in Bengali..."
        export LANGUAGE=bn
        ;;
    24)
        echo "Starting in Korean..."
        export LANGUAGE=ko
        ;;
    25)
        echo "Starting in Vietnamese..."
        export LANGUAGE=vi
        ;;
    26)
        echo "Starting in Thai..."
        export LANGUAGE=th
        ;;
    27)
        echo "Starting in Indonesian..."
        export LANGUAGE=id
        ;;
    28)
        echo "Starting in Swedish..."
        export LANGUAGE=sv
        ;;
    29)
        echo "Starting in Norwegian..."
        export LANGUAGE=nb
        ;;
    30)
        echo "Starting in Finnish..."
        export LANGUAGE=fi
        ;;
    31)
        echo "Starting in Romanian..."
        export LANGUAGE=ro
        ;;
    32)
        echo "Starting in Czech..."
        export LANGUAGE=cs
        ;;
    33)
        echo "Starting in Persian..."
        export LANGUAGE=fa
        export LANG=fa_IR.UTF-8
        ;;
    1|"")
        echo "Starting in English..."
        unset LANGUAGE
        ;;
    *)
        echo "Invalid choice. Using English (default)."
        unset LANGUAGE
        ;;
esac

# Run the application
./target/debug/loremgenerator "$@"
