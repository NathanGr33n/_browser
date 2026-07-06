#!/usr/bin/env bash
# _Browser — build/run helper around mach.
#
# Usage:
#   build.sh debug      Build a debug browser
#   build.sh release    Build an optimized browser
#   build.sh run        Run the most recently built browser (debug mozconfig by default)
#   build.sh package    Package the most recent build (release mozconfig by default)
#
# Override the Firefox source location with SRC_DIR
# (default: $HOME/mozilla-source/firefox).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
KIT_DIR="$(dirname "$SCRIPT_DIR")"
SRC_DIR="${SRC_DIR:-$HOME/mozilla-source/firefox}"

MOZCONFIG_DEBUG="$KIT_DIR/mozconfig/mozconfig.linux.debug"
MOZCONFIG_RELEASE="$KIT_DIR/mozconfig/mozconfig.linux.release"

if [ ! -x "$SRC_DIR/mach" ]; then
  echo "ERROR: mach not found in $SRC_DIR. Run setup.sh first, or set SRC_DIR." >&2
  exit 1
fi

cmd="${1:-debug}"
cd "$SRC_DIR"

case "$cmd" in
  debug)
    export MOZCONFIG="$MOZCONFIG_DEBUG"
    echo "==> Building debug (MOZCONFIG=$MOZCONFIG)"
    exec ./mach build ;;
  release)
    export MOZCONFIG="$MOZCONFIG_RELEASE"
    echo "==> Building release (MOZCONFIG=$MOZCONFIG)"
    exec ./mach build ;;
  run)
    export MOZCONFIG="${MOZCONFIG:-$MOZCONFIG_DEBUG}"
    echo "==> Running (MOZCONFIG=$MOZCONFIG)"
    exec ./mach run ;;
  package)
    export MOZCONFIG="${MOZCONFIG:-$MOZCONFIG_RELEASE}"
    echo "==> Packaging (MOZCONFIG=$MOZCONFIG)"
    exec ./mach package ;;
  *)
    echo "Usage: $0 {debug|release|run|package}" >&2
    exit 1 ;;
esac
