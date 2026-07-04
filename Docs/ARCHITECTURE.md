# _Browser — Architecture

## Project Direction

_Browser is a **Gecko-based browser fork**, built on top of Mozilla Firefox (ESR).
The goal is NOT to build a browser engine from scratch, but to build a polished, highly
customizable browser experience on top of the proven Gecko engine — similar in spirit to
[Zen Browser](https://zen-browser.app/).

The existing Rust engine code in `src/` is from a prior from-scratch approach and is
**superseded** by this direction. It may be referenced for learning but is not the implementation target.

---

## What "Gecko-Based" Means

Gecko is Mozilla's browser engine powering Firefox. Building on it means:

- **We fork Firefox** at a specific ESR (Extended Support Release) version.
- **We do not touch the engine itself** (layout, JS engine, networking). Those are Gecko's domain.
- **We own the browser chrome** — the UI layer built in XUL/HTML/CSS/JS that sits on top of Gecko.
- All UI/UX improvements, customization, and new features live in the chrome layer.

This is exactly how Zen Browser, LibreWolf, Floorp, and other Firefox forks work.

---

## Firefox Source Structure (What We Work In)

After forking Firefox, the key directories are:

```
firefox-source/
├── browser/                    ← PRIMARY: Browser chrome (UI layer) — we own this
│   ├── base/
│   │   ├── content/
│   │   │   ├── browser.xhtml   ← Main browser window definition
│   │   │   ├── tabbrowser.js   ← Tab management logic
│   │   │   ├── browser.js      ← Core browser JS
│   │   │   └── browser.css     ← Browser chrome styles
│   │   └── skin/               ← Default CSS for browser chrome
│   ├── themes/
│   │   ├── linux/              ← Linux-specific styles (our primary target)
│   │   ├── osx/
│   │   └── windows/
│   ├── components/             ← Browser services (bookmarks, downloads, history)
│   ├── extensions/             ← Built-in extensions (screenshots, pocket, etc.)
│   └── locales/                ← i18n strings
├── toolkit/                    ← Toolkit layer (shared UI primitives)
│   ├── content/                ← Dialogs, panels, basic elements
│   └── themes/                 ← Cross-platform base theme
├── dom/                        ← Gecko DOM (C++/Rust) — do not modify
├── layout/                     ← Gecko layout engine (C++/Stylo Rust) — do not modify
├── js/                         ← SpiderMonkey JS engine — do not modify
├── gfx/                        ← Graphics/compositing — do not modify
├── netwerk/                    ← Network layer — do not modify
├── security/                   ← NSS/TLS — do not modify
└── build/                      ← Build system (mach, configure, moz.build)
```

**Rule of thumb:** If it's in `browser/` or `toolkit/`, it's UI work we can do. Everything else is Gecko internals — only touch with strong justification.

---

## Chrome vs. Content

Firefox has a critical split:

| Layer      | What it is                                          | Technologies          |
|------------|-----------------------------------------------------|-----------------------|
| **Chrome** | Browser UI (tabs, address bar, sidebar, menus, toolbars) | XUL, HTML, CSS, JS    |
| **Content**| Web pages rendered by Gecko                         | HTML/CSS/JS (untrusted)|

We work exclusively in the **Chrome** layer. Chrome has elevated privileges and can access
Firefox APIs directly. Content runs in sandboxed processes.

The chrome entry point is `browser/base/content/browser.xhtml`. This is the main browser window.

---

## UI Customization Approach

### CSS Overrides
The fastest and most maintainable way to restyle the browser is CSS:
- `browser/themes/linux/browser.css` — Platform-specific overrides
- `browser/base/skin/browser.css` — Global browser chrome styles
- CSS custom properties (`--color-accent`, `--toolbar-height`, etc.) for theming

### JavaScript Components
New UI features are implemented as JS modules in `browser/components/`:
- Each feature gets its own subdirectory
- Registered via `moz.build` files
- Can use all Firefox-internal APIs (nsIBrowserService, Places, etc.)

### XUL Modifications
For structural UI changes (adding panels, changing the toolbar layout, sidebar):
- Modify `browser/base/content/browser.xhtml` for element additions
- Use XUL overlays/fragments for modular additions

### User Preferences
Custom settings are exposed via:
- `browser/app/profile/firefox.js` — Default preferences
- `browser/components/preferences/` — Settings UI
- `Services.prefs` API in JS for dynamic reads/writes

---

## Customization Framework Design

The goal is a layered customization system:

```
User Settings (prefs)
    ↓
Theme Layer (CSS variables + swappable theme files)
    ↓
Layout Options (compact/normal/expanded, vertical/horizontal tabs)
    ↓
Feature Toggles (sidebar panels, split view, etc.)
    ↓
Base Chrome (browser.xhtml + core CSS)
```

### Theming
- Define all colors/sizes as CSS custom properties on `:root`
- Ship built-in theme presets as CSS files that override those variables
- Allow users to pick themes from Settings or via a theme picker in the toolbar

### Layout Modes
- Tabs: Horizontal (default), Vertical (sidebar-style), Floating
- Toolbar: Full, Compact, Hidden
- Sidebar: Collapsed, Icon-only, Expanded

---

## Build System

Firefox uses its own build system (`mach`):

```bash
# Configure (run once)
./mach configure

# Build (debug)
./mach build

# Build (release/optimized)
./mach build -r

# Run the built browser
./mach run

# Run tests
./mach test browser/base/content/test/
```

`.mozconfig` file at the root controls build settings:
```
# .mozconfig example
ac_add_options --enable-application=browser
ac_add_options --enable-debug          # for dev builds
ac_add_options --disable-tests         # speeds up builds if not testing
mk_add_options MOZ_OBJDIR=@TOPSRCDIR@/obj-x86_64-linux-gnu
```

---

## Reference: How Zen Browser Implements Key Features

Zen Browser (https://github.com/zen-browser/desktop) is the closest model to follow:

| Feature               | Implementation in Zen                                      |
|-----------------------|------------------------------------------------------------|
| Vertical tabs         | JS component + CSS, modifies `tabbrowser.js`               |
| Workspaces            | New component in `browser/components/zen-workspaces/`      |
| Compact mode          | CSS media-query-style rules on `[zen-compact-mode]` attr   |
| Sidebar panels        | New XUL panel + sidebar service wrapper                    |
| Custom new tab        | Custom `about:newtab` page override                        |
| Branding              | `browser/branding/` directory with icons/strings           |

Study Zen's `browser/components/` directory structure as the primary reference for how to add features.

---

## Key Firefox APIs Available in Chrome

| API                         | Purpose                                               |
|-----------------------------|-------------------------------------------------------|
| `gBrowser`                  | Tab/browser management (`gBrowser.addTab()`, etc.)    |
| `Services.prefs`            | Read/write preferences                                |
| `Services.obs`              | Subscribe to browser events (observer pattern)        |
| `PlacesUtils`               | History and bookmarks                                 |
| `ChromeUtils.import()`      | Import ES modules from chrome code                    |
| `BrowserWindowTracker`      | Track open windows                                    |
| `SessionStore`              | Tab session save/restore                              |

---

## Current Repo State

The `src/` directory contains the old from-scratch Rust engine (Phases 1–8, 277 tests).
This code is **not the active implementation target** for the Gecko-based direction.
The `core-change` branch marks the pivot point.

Going forward, the implementation work will be in a Firefox fork repository. This Rust
codebase may be archived, or kept for reference/educational purposes.
