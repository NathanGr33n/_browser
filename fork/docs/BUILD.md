# Building _Browser

_Browser is a fork of **Firefox ESR 140**. This guide covers producing a local
build using the setup kit in `fork/`.

The workflow is:
1. Clone the upstream Firefox source (large).
2. Apply the _Browser overlay (branding + default-pref strips).
3. Install build dependencies with `mach bootstrap`.
4. Build and run with `mach` via `build.sh`.

Steps 1–3 are automated by `scripts/setup.sh`.

## Platform
The primary target is **Linux** (Zorin OS / Ubuntu-based). On Windows, build
inside **WSL2 (Ubuntu)**. Native Windows builds are possible with MozillaBuild
but are not the supported path here (the mozconfigs are Linux-targeted).

> Tip: clone this repository *inside* Linux/WSL2. The scripts are shell scripts
> and must keep LF line endings; a fresh clone in Linux guarantees that. Avoid
> running them from a Windows working copy over `/mnt/...`, where they may carry
> CRLF endings.

## Prerequisites
- ~40 GB+ free disk space for the source and build artifacts.
- 8 GB RAM minimum; 16 GB+ strongly recommended (low RAM may require disabling
  optimization — see Troubleshooting).
- `git` and `python3` available on `PATH`.
- Everything else (compilers, Rust, etc.) is installed by `mach bootstrap`.

## Build steps
```bash
cd fork

# 1–3: clone esr140, apply overlay, install deps.
./scripts/setup.sh

# 4: build.
./scripts/build.sh debug      # or: release

# Run the build.
./scripts/build.sh run
```
By default the Firefox checkout is created at `~/mozilla-source/firefox`.

## Configuration (environment variables)
- `ESR_BRANCH` — Firefox ESR branch to build (default `esr140`).
- `SRC_PARENT` — directory that will contain `firefox/` (default `~/mozilla-source`).
- `FIREFOX_REPO` — Firefox git URL (default `https://github.com/mozilla-firefox/firefox.git`).
- `SRC_DIR` — used by `build.sh` to locate the checkout (default `~/mozilla-source/firefox`).

Example: `SRC_PARENT=/data/src ./scripts/setup.sh`

## What the overlay changes
`scripts/apply-overlay.sh` seeds `browser/branding/_browser/` from Firefox's
built-in `unofficial` branding (for a valid `moz.build` and placeholder icons),
then copies the files in `overlay/` on top:
- Brand strings → `_Browser` (`brand.ftl`, `brand.dtd`, `brand.properties`).
- `MOZ_APP_DISPLAYNAME=_Browser` (`configure.sh`).
- Default prefs that strip built-ins (`pref/firefox-branding.js`): Pocket off,
  telemetry/data reporting off, Firefox Accounts/Sync off by default.

The mozconfigs select this branding via `--with-branding=browser/branding/_browser`.

## Replacing the app icon
The seeded icons are Firefox placeholders. Replace them with _Browser artwork,
keeping the original filenames. See
`overlay/browser/branding/_browser/README.md` for the full list.

### Linux/Wayland icon association (optional)
On Wayland the app icon binds to the window class. If the icon does not appear,
set a stable remoting name by adding this line to the chosen mozconfig, and make
the desktop entry's `StartupWMClass` match:
```
MOZ_APP_REMOTINGNAME=_browser
```

## Baseline commit (Phase 1 deliverable)
Once `./scripts/build.sh debug` succeeds and `./scripts/build.sh run` launches a
browser showing the _Browser name (no Firefox branding), commit the baseline in
the **Firefox checkout** (a separate repository from this one):
```bash
cd ~/mozilla-source/firefox
git add browser/branding/_browser
git commit -m "feat-> Add _Browser branding baseline (esr140)"
```

## Troubleshooting
- **`mach bootstrap` prompts:** answer the build-type prompt with Firefox for
  Desktop; decline telemetry if you prefer.
- **Out of memory during build:** in `mozconfig.linux.release`, comment out
  `ac_add_options --enable-optimize`, or reduce parallelism with
  `mk_add_options MOZ_MAKE_FLAGS="-j4"`.
- **`/bin/bash^M: bad interpreter`:** the scripts have CRLF endings. Re-clone the
  repo inside Linux/WSL2, or run `sed -i 's/\r$//' scripts/*.sh`.
- **Clone is too large:** add `--depth=1` to the clone by setting, e.g.,
  a shallow clone manually, or accept the full single-branch history.

## Native Windows (not supported here)
If you must build on Windows without WSL2, follow Mozilla's
"Building Firefox On Windows" docs with MozillaBuild, then point `MOZCONFIG` at a
Windows-appropriate config (the Linux mozconfigs here will not apply). This path
is untested for _Browser.
