# Browser Engine

A performance-focused browser engine built from scratch in Rust.

## Project Status

### Phase 1: Core Foundation ✓ COMPLETE

**HTML/CSS Parser & Layout Engine**

- ✓ DOM tree structure with elements, text nodes, and attributes
- ✓ HTML5 parser using html5ever
- ✓ CSS parser with selectors (tag, class, id) and declarations
- ✓ Style computation system with cascade and specificity
- ✓ Layout engine implementing CSS box model
- ✓ Basic block layout algorithm
- ✓ Unit tests for all components (13 tests passing)

## Current Capabilities

The browser engine can currently:

1. **Parse HTML** - Convert HTML strings into a DOM tree
2. **Parse CSS** - Parse stylesheets with selectors and property declarations
3. **Compute Styles** - Match CSS rules to DOM elements with proper specificity
4. **Calculate Layout** - Compute box dimensions using the CSS box model

## Running the Demo

```bash
cargo run
```

## Running Tests

```bash
cargo test --lib
```

## Running Tests

**Window Test** (shows blue-gray background):
```bash
cargo run --bin window_test
```

**Rectangle Test** (shows colored rectangles):
```bash
cargo run --bin rect_test
```

## Project Structure

```
src/
├── dom/          # DOM tree representation
├── html/         # HTML parser (html5ever integration)
├── css/          # CSS parser and value types
├── style/        # Style computation and selector matching
├── layout/       # Layout engine with box model
├── window/       # Window management (Phase 2)
├── renderer/     # GPU renderer (Phase 2)
├── lib.rs        # Library interface
├── main.rs       # Demo application
└── bin/          # Test binaries
```

## Technology Stack

- **Language**: Rust (edition 2021)
- **HTML Parser**: html5ever
- **CSS Parser**: cssparser
- **Selector Matching**: selectors crate

### Phase 2: Rendering Pipeline (In Progress)

**Window & Graphics Setup** ✓

- ✓ Cross-platform window creation (winit)
- ✓ GPU renderer initialization (wgpu)
- ✓ Hardware-accelerated clear operations
- ✓ Window resize handling
- ✓ Event loop integration

**Display List & Rectangle Rendering** ✓

- ✓ Display list generation from layout tree
- ✓ WGSL shader pipeline for rectangles
- ✓ GPU-accelerated rectangle rendering
- ✓ Color and alpha blending support
- ✓ Viewport culling

**Next Steps:**
- Border rendering
- Text rendering with font support
- Image decoding and display
- Scrolling support

## Architecture

The browser follows a traditional rendering pipeline:

```
HTML → DOM Tree → Style Tree → Layout Tree → Display List → Pixels
```

**Current Implementation:**
- HTML → DOM Tree ✓
- DOM + CSS → Style Tree ✓
- Style → Layout Tree ✓
- Layout → Display List (Phase 2)
- Display List → Pixels (Phase 2)

## License

This is an educational project for learning browser engine architecture.
