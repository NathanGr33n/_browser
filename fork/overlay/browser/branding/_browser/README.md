# _Browser branding

This directory is applied into a Firefox source tree at
`browser/branding/_browser/` by `fork/scripts/apply-overlay.sh`.

## How it is assembled
The apply script first *seeds* this directory by copying Firefox's built-in
`browser/branding/unofficial/` branding (which provides a known-good `moz.build`),
then copies the files in this overlay on top:

- `configure.sh` — sets `MOZ_APP_DISPLAYNAME` to `_Browser`.
- `locales/en-US/brand.ftl`, `brand.dtd`, `brand.properties` — brand strings.
- `pref/firefox-branding.js` — default prefs, including the Phase 1 built-in strips.
- `_browser.svg` plus the `default*.png`, `mozicon128.png`, and `content/*.png`
  icons — the `_Browser` placeholder app icons (see below).

The overlay keeps the seeded `moz.build` untouched and reuses its icon file names,
so all references stay valid.

## App icon (Phase 1 "app icon")
This overlay ships generated `_Browser` placeholder icons — an indigo badge with an
underscore mark, rasterized from `_browser.svg` and intentionally distinct from
Firefox branding. Replace them with final `_Browser` artwork later, keeping the same
file names so `moz.build` keeps resolving them:

- `default16.png`, `default32.png`, `default48.png`, `default64.png`, `default128.png`
  — Linux/GTK window and app icons.
- `content/about-logo.png`, `content/about-logo@2x.png` — About dialog logo (1x + 2x).
- `content/icon64.png` — miscellaneous UI.
- `mozicon128.png` — installer/desktop icon.

The macOS (`firefox.icns`) and Windows (`firefox.ico`, `document.ico`) icons are left
as seeded from `unofficial`; only replace them if/when those platforms are targeted
(the primary target is Linux).
