# _Browser — Roadmap

## Phase 1: Fork Setup (Weeks 1–4)
**Goal:** Buildable Firefox fork with custom branding.

- [ ] Fork Firefox ESR (latest stable ESR release)
- [ ] Set up Linux build environment (build deps, `mach bootstrap`)
- [ ] Create `.mozconfig` for debug + release builds
- [ ] Successful `./mach build` + `./mach run`
- [ ] Custom branding: name, about page, app icon (`browser/branding/_browser/`)
- [ ] Strip unwanted built-ins (Pocket, Firefox Sync default, telemetry)
- [ ] Commit baseline build

**Success criteria:** Browser launches, shows custom name/icon, no Firefox branding.

---

## Phase 2: Core UI Overhaul (Weeks 5–12)
**Goal:** Visually distinct, cleaner UI than stock Firefox.

- [ ] Establish CSS variable system (`--browser-bg`, `--accent-color`, `--tab-height`, etc.)
- [ ] Redesign tab bar (shape, spacing, active/hover states)
- [ ] Redesign address bar (unified bar style, rounded corners, cleaner dropdowns)
- [ ] Redesign toolbar (icon style, spacing, button states)
- [ ] Custom new tab page (`about:newtab` override — clean, minimal)
- [ ] Sidebar base styling (width, collapse animation, icon strip)
- [ ] Remove visual clutter (megabar, promos, default icons we don't want)

**Success criteria:** UI is visually cohesive and clearly different from stock Firefox.

---

## Phase 3: Customization Framework (Weeks 13–20)
**Goal:** Users can meaningfully personalize the browser.

- [ ] Theme system: CSS variable presets (Light, Dark, + 2–3 accent themes)
- [ ] Theme picker UI (toolbar button or Settings panel)
- [ ] Tab layout toggle: Horizontal ↔ Vertical tabs (sidebar-style)
- [ ] Toolbar density: Normal / Compact
- [ ] Sidebar: collapsible, icon-only mode, expanded mode
- [ ] Settings page additions (`browser/components/preferences/`) for all above options
- [ ] Persist all options via `Services.prefs`

**Success criteria:** User can switch themes, tab layout, and density; settings persist across restarts.

---

## Phase 4: Feature Additions (Weeks 21–36)
**Goal:** Differentiated features beyond visual polish.

- [ ] **Tab Workspaces** — group tabs into named workspaces, switch with keyboard shortcut
- [ ] **Split View** — view two tabs side-by-side in one window
- [ ] **Sidebar Panels** — bookmarks, history, notes as collapsible sidebar panels
- [ ] **Focus Mode** — hide all chrome except content + minimal exit button
- [ ] **Reader Mode improvements** — custom fonts, line-width, sepia/dark options
- [ ] **Custom start page** — configurable widgets (bookmarks, clock, search)

Implement each as its own component under `browser/components/_browser-<feature>/`.

**Success criteria:** Each feature is functional, togglable, and non-breaking when disabled.

---

## Phase 5: Polish & Distribution (Weeks 37+)
**Goal:** Releasable build for Linux.

- [ ] Release `.mozconfig` (optimizations, PGO if feasible)
- [ ] AppImage packaging
- [ ] `.deb` package for Ubuntu/Zorin
- [ ] Flatpak manifest
- [ ] Update infrastructure (diff-based or full-bundle)
- [ ] Basic CI (GitHub Actions: build check on push)
- [ ] Public README rewrite and project website (optional)

---

## Reference Projects
- [Zen Browser](https://github.com/zen-browser/desktop) — closest model, study `browser/components/`
- [Floorp](https://github.com/Floorp-Projects/Floorp) — heavy customization, good reference for prefs system
- [LibreWolf](https://codeberg.org/librewolf/source) — minimal fork, clean `.mozconfig` reference
