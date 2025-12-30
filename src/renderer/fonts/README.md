# Font Files

Text rendering requires font files for glyph rasterization.

## Status

Currently **stubbed** - text rendering infrastructure is in place but requires font files.

## Implementation Options

### Option 1: Embedded Font (Recommended for MVP)
- Add a small TTF font file (e.g., DejaVuSans.ttf, ~300KB)
- Embed in binary with `include_bytes!`
- Pros: Self-contained, no dependencies
- Cons: Increases binary size

### Option 2: System Fonts
- Use `font-kit` crate to discover system fonts
- Load fonts at runtime
- Pros: No binary bloat, native fonts
- Cons: Platform-specific, requires font-kit dependency

### Option 3: Web Fonts
- Download fonts from Google Fonts or similar
- Cache locally
- Pros: Large selection, up-to-date
- Cons: Network dependency, licensing

## Next Steps

1. Choose font strategy
2. Add font file(s) to this directory
3. Update `text_renderer.rs` to load actual fonts
4. Implement glyph-to-texture pipeline
5. Create text rendering shader (with texture sampling)
6. Wire up to display list

## Current Stub Behavior

- `measure_text()` returns monospace approximation
- `rasterize_glyph()` creates placeholder glyphs
- No actual rendering occurs

This allows the rest of the engine to compile and work while text is pending.
