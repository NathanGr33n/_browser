#!/usr/bin/env bash
# _Browser — copy the fork overlay into a Firefox source tree.
#
# Usage: apply-overlay.sh /path/to/firefox
#
# Seeds browser/branding/_browser from Firefox's built-in "unofficial" branding
# (which provides a known-good moz.build and placeholder icons), then copies the
# _Browser overlay files on top.
set -euo pipefail

SRC_DIR="${1:-}"
if [ -z "$SRC_DIR" ] || [ ! -d "$SRC_DIR/browser" ]; then
  echo "ERROR: pass the path to a Firefox source tree (must contain browser/)." >&2
  echo "Usage: $0 /path/to/firefox" >&2
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
KIT_DIR="$(dirname "$SCRIPT_DIR")"
OVERLAY_DIR="$KIT_DIR/overlay"
BRANDING_SRC="$SRC_DIR/browser/branding/unofficial"
BRANDING_DST="$SRC_DIR/browser/branding/_browser"

if [ ! -d "$OVERLAY_DIR" ]; then
  echo "ERROR: overlay directory not found at $OVERLAY_DIR" >&2
  exit 1
fi

# 1. Seed branding/_browser from branding/unofficial (only if not already present).
if [ ! -d "$BRANDING_DST" ]; then
  if [ ! -d "$BRANDING_SRC" ]; then
    echo "ERROR: seed branding not found at $BRANDING_SRC" >&2
    exit 1
  fi
  echo "==> Seeding branding/_browser from branding/unofficial"
  cp -a "$BRANDING_SRC" "$BRANDING_DST"
fi

# 2. Copy overlay files over the source tree (branding text/prefs, etc.).
echo "==> Copying overlay files into $SRC_DIR"
cp -a "$OVERLAY_DIR/." "$SRC_DIR/"

echo "==> Overlay applied. Branding directory: $BRANDING_DST"
