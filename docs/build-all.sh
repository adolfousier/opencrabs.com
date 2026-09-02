#!/usr/bin/env bash
# Build the OpenCrabs docs in English + all translated locales.
# Usage: ./build-all.sh   (from the docs/ directory)
#
# Locales live in po/<locale>.po (gettext). A locale dir without a .po file
# is skipped. Missing translations fall back to English per-paragraph, so a
# partially translated book never renders broken.
set -euo pipefail
cd "$(dirname "$0")"

LOCALES=(pt-PT fr ru id)

echo "==> Building English book"
mdbook build -d book

for loc in "${LOCALES[@]}"; do
    if [ -f "po/${loc}.po" ]; then
        echo "==> Building ${loc} book"
        MDBOOK_BOOK__language="${loc}" mdbook build -d "book/${loc}"
    else
        echo "==> Skipping ${loc} (no po/${loc}.po)"
    fi
done

echo "==> All books built:"
ls -d book book/* 2>/dev/null | grep -v '^book/$' || true
