#!/bin/bash
# Script to run Lorem Ipsum Generator with language selection

echo "Lorem Ipsum Generator - Language Selection"
echo "=========================================="
echo "Available languages:"
echo "  1) English (default)"
echo "  2) Spanish (Español)"
echo "  3) German (Deutsch)"
echo "  4) French (Français)"
echo "  5) Portuguese (Português - Brasil)"
echo "  6) Italian (Italiano)"
echo "  7) Russian (Русский)"
echo "  8) Japanese (日本語)"
echo ""
read -p "Select language [1-8, default: 1]: " choice

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
        echo "Starting in Italian..."
        export LANGUAGE=it
        ;;
    7)
        echo "Starting in Russian..."
        export LANGUAGE=ru
        ;;
    8)
        echo "Starting in Japanese..."
        export LANGUAGE=ja
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
