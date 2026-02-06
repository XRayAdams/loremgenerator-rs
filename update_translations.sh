#!/bin/bash
# Script to extract translatable strings and update translation files

echo "Extracting translatable strings..."

# Extract strings from source code
xgettext --from-code=UTF-8 \
  --language=C \
  --keyword=tr! \
  --keyword=gettext \
  --package-name=loremgenerator \
  --package-version=2.2.7 \
  --msgid-bugs-address="https://github.com/XRayAdams/loremgenerator-rs/issues" \
  --output=po/loremgenerator.pot \
  --no-wrap \
  src/*.rs src/helpers/*.rs

if [ $? -eq 0 ]; then
  echo "Successfully extracted strings to po/loremgenerator.pot"
  
  # Update existing translations
  echo ""
  echo "Updating existing translations..."
  for po_file in po/*.po; do
    if [ -f "$po_file" ]; then
      lang=$(basename "$po_file" .po)
      echo "  Updating $lang translation..."
      msgmerge --update --no-wrap "$po_file" po/loremgenerator.pot
    fi
  done
  
  echo ""
  echo "Translation update complete!"
  echo "Please review and update the translation files in po/ directory."
else
  echo "Error: xgettext failed. Make sure gettext tools are installed."
  echo "On Ubuntu/Debian: sudo apt install gettext"
  exit 1
fi
