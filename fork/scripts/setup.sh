#!/usr/bin/env bash
# _Browser — set up a Firefox ESR fork build tree.
#
# Clones the official Firefox git repo, switches to the ESR branch, applies the
# _Browser overlay (branding + pref strips), and installs build dependencies.
#
# WARNING: this clones a very large repository and `mach bootstrap` installs
# system build dependencies. The first build afterwards can take a long time.
#
# Override defaults via environment variables:
#   ESR_BRANCH    Firefox ESR branch to build     (default: esr140)
#   SRC_PARENT    Dir that will hold "firefox/"    (default: $HOME/mozilla-source)
#   FIREFOX_REPO  Git URL of the Firefox source    (default: Mozilla's official repo)
set -euo pipefail

ESR_BRANCH="${ESR_BRANCH:-esr140}"
SRC_PARENT="${SRC_PARENT:-$HOME/mozilla-source}"
FIREFOX_REPO="${FIREFOX_REPO:-https://github.com/mozilla-firefox/firefox.git}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="$SRC_PARENT/firefox"

echo "==> _Browser fork setup"
echo "    Repo       : $FIREFOX_REPO"
echo "    ESR branch : $ESR_BRANCH"
echo "    Source dir : $SRC_DIR"

for tool in git python3; do
  command -v "$tool" >/dev/null 2>&1 || { echo "ERROR: '$tool' is required but was not found." >&2; exit 1; }
done

mkdir -p "$SRC_PARENT"

if [ -d "$SRC_DIR/.git" ]; then
  echo "==> Source already present; fetching latest for $ESR_BRANCH"
  git -C "$SRC_DIR" fetch origin "$ESR_BRANCH"
else
  echo "==> Cloning Firefox source (large download; this may take a while)"
  git clone --single-branch --branch "$ESR_BRANCH" "$FIREFOX_REPO" "$SRC_DIR"
fi

cd "$SRC_DIR"
git switch "$ESR_BRANCH" 2>/dev/null || git switch -c "$ESR_BRANCH" --track "origin/$ESR_BRANCH"

echo "==> Applying _Browser overlay"
"$SCRIPT_DIR/apply-overlay.sh" "$SRC_DIR"

echo "==> Installing build dependencies (./mach bootstrap)"
echo "    (You may be prompted; decline telemetry if you prefer.)"
./mach bootstrap --application-choice browser

cat <<EOF
==> Setup complete.
Next steps:
  $SCRIPT_DIR/build.sh debug     # build a debug browser
  $SCRIPT_DIR/build.sh release   # build an optimized browser
  $SCRIPT_DIR/build.sh run       # run the most recent build
EOF
