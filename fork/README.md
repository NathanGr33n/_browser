# _Browser fork setup kit

_Browser is a Gecko-based browser fork built on **Firefox ESR** (in the spirit of
Zen Browser), focused on UI/UX and a customization framework.

This `fork/` directory is the **setup kit** that turns an upstream Firefox ESR
checkout into a _Browser build. The large Firefox source tree itself is **not**
stored in this repo — it is cloned separately by `scripts/setup.sh`.

See `../Docs/ARCHITECTURE.md` and `../Docs/ROADMAP.md` for the overall direction
and phased plan.

## Contents
- `mozconfig/` — Linux build configs: `mozconfig.linux.debug`, `mozconfig.linux.release`.
- `overlay/` — files copied into the Firefox tree (branding + default-pref overrides).
- `scripts/` — `setup.sh`, `apply-overlay.sh`, `build.sh`.
- `docs/BUILD.md` — full build instructions plus WSL2/Windows notes and troubleshooting.

## Quick start (Linux / WSL2)
```bash
# Clone THIS repo on Linux or WSL2 so files keep LF endings, then:
cd fork
./scripts/setup.sh          # clone Firefox esr140, apply overlay, install build deps
./scripts/build.sh debug    # first build (large + slow)
./scripts/build.sh run      # launch _Browser
```
Read `docs/BUILD.md` first for prerequisites (disk/RAM), configuration options,
and how to replace the placeholder app icons.

## Status
Phase 1: fork setup, custom branding, and stripped built-ins (Pocket, telemetry,
Sync-by-default). The large clone and first build are intentionally deferred —
run `scripts/setup.sh` when you are ready. Later phases (UI overhaul, customization
framework, features, packaging) build on top of this baseline.
