# _Browser branding

This directory is applied into a Firefox source tree at
`browser/branding/_browser/` by `fork/scripts/apply-overlay.sh`.

## How it is assembled
The apply script first *seeds* this directory by copying Firefox's built-in
`browser/branding/unofficial/` branding (which provides a known-good `moz.build`
and a placeholder icon set), then copies the files in this overlay on top:

- `configure.sh` — sets `MOZ_APP_DISPLAYNAME` to `_Browser`.
- `locales/en-US/brand.ftl`, `brand.dtd`, `brand.properties` — brand strings.
- `pref/firefox-branding.js` — default prefs, including the Phase 1 built-in strips.

Because only these text files are overridden, the seeded `moz.build` and icon
filenames stay valid.

## Icons to replace later (Phase 1 "app icon")
The seeded icons are Firefox placeholders. Replace them with `_Browser` artwork,
keeping the same filenames so `moz.build` keeps resolving them:

- `default16.png`, `default32.png`, `default48.png`, `default64.png`, `default128.png`
  — Linux/GTK window and app icons.
- `content/about-logo.png`, `content/about-logo@2x.png` — About dialog logo.
- `content/icon64.png` — miscellaneous UI.
- `mozicon128.png` — installer/desktop icon.
- `firefox.icns` (macOS) and `firefox.ico` / `document.ico` (Windows) — only needed
  if/when those platforms are targeted.

Provide 1x and 2x (HiDPI) variants where applicable.
