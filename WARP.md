# _Browser — Project Context for AI

## What This Project Is
A **Gecko-based browser fork** (like Zen Browser), built on top of Firefox ESR.
Focus: UI/UX improvements and a rich customization framework.
**Not** a from-scratch engine — the `src/` Rust code is from a prior abandoned direction.

## Current State
- Branch `core-change`: project pivot committed, docs being written
- `src/`: old from-scratch Rust engine — **ignore for implementation work**
- `Docs/CoreIdea.md`: new vision (Gecko-based, Zen-like)
- `Docs/ARCHITECTURE.md`: Firefox fork architecture, what files to modify
- `Docs/ROADMAP.md`: phased implementation plan (5 phases)

## Active Implementation Target
Firefox fork repository (not yet created). When created, it will be a separate repo
cloned from Firefox ESR. All browser chrome work happens there.

## Key Rules for Implementation
- Work only in `browser/` and `toolkit/` — never modify Gecko internals (`dom/`, `layout/`, `js/`, `gfx/`, `netwerk/`)
- All UI changes use CSS custom properties on `:root` — no hardcoded colors/sizes
- Each new feature is its own component: `browser/components/_browser-<feature>/`
- All user options persist via `Services.prefs`
- Primary platform target: **Linux** (Zorin OS / Ubuntu-based)
- Primary reference: [Zen Browser source](https://github.com/zen-browser/desktop)

## Tech Stack
- **Engine:** Gecko (Firefox ESR)
- **Chrome layer:** XUL, HTML, CSS, JavaScript (ES modules)
- **Build system:** `mach` + `.mozconfig`
- **Language for new components:** JavaScript (chrome-privileged)
- **Styling:** CSS custom properties + platform theme overrides
