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

### Phase 2: Rendering Pipeline ✓ COMPLETE

**Full HTML → Pixels Pipeline Working!**

- ✓ GPU-accelerated rendering (wgpu)
- ✓ Rectangle and border rendering
- ✓ Display list generation
- ✓ Scrolling infrastructure
- ✓ **End-to-end demo: HTML+CSS to screen**

### Phase 3: Networking ✓ COMPLETE

**Fetch and Render Real Websites!**

- ✓ HTTP client with reqwest (blocking API)
- ✓ Resource loader with 50MB LRU cache
- ✓ Page loader integrating HTTP + parsing
- ✓ CSS extraction from `<style>` and `<link>` tags
- ✓ Navigation history (back/forward)
- ✓ **Network demo: fetches example.com and renders it**
- ✓ 37 unit tests passing

## Current Capabilities

The browser engine can currently:

1. **Fetch HTML from the Web** - HTTP client with caching
2. **Parse HTML** - Convert HTML strings into a DOM tree
3. **Parse CSS** - Parse stylesheets with selectors and property declarations
4. **Compute Styles** - Match CSS rules to DOM elements with proper specificity
5. **Calculate Layout** - Compute box dimensions using the CSS box model
6. **Render to Screen** - GPU-accelerated drawing of rectangles and borders

## Running the Demo

```bash
cargo run
```

## Running Tests

```bash
cargo test --lib
```

## Running Demos

**Network Demo** (fetch real websites):
```bash
cargo run --example network_demo
```
Fetches example.com and httpbin.org, demonstrating the full HTTP → Render pipeline!

**Full Browser Demo** (HTML+CSS rendering):
```bash
cargo run --bin browser_demo
```
This demonstrates the complete pipeline from HTML/CSS to pixels!

## Component Tests

**Window Test** (blue-gray background):
```bash
cargo run --bin window_test
```

**Rectangle Test** (colored shapes):
```bash
cargo run --bin rect_test
```

**Border Test** (boxes with borders):
```bash
cargo run --bin border_test
```

## Project Structure

```
src/
├── dom/          # DOM tree representation
├── html/         # HTML parser (html5ever integration)
├── css/          # CSS parser and value types
├── style/        # Style computation and selector matching
├── layout/       # Layout engine with box model
├── display/      # Display list generation
├── window/       # Window management
├── renderer/     # GPU renderer with wgpu
├── net/          # HTTP client and resource loading (Phase 3)
├── lib.rs        # Library interface
├── main.rs       # Demo application
└── bin/          # Test binaries
```

## Technology Stack

- **Language**: Rust (edition 2021)
- **HTML Parser**: html5ever
- **CSS Parser**: cssparser
- **Networking**: reqwest (blocking API)
- **URL Parsing**: url crate
- **Graphics**: wgpu (WebGPU API)
- **Window**: winit (cross-platform)

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

**Border Rendering** ✓

- ✓ Per-edge border width support
- ✓ Border color customization
- ✓ Efficient edge-based rendering
- ✓ Combined rect+border rendering

**Text Rendering** (Infrastructure Ready)

- ✓ Glyph caching system
- ✓ Texture atlas management
- ✓ Text measurement API
- ✓ Layout positioning
- ⚠ Pending: Font file integration

**Scrolling** ✓

- ✓ Scroll state management
- ✓ Viewport offset tracking
- ✓ Content size handling
- ✓ Scroll clamping

**End-to-End Integration** ✓

- ✓ Full HTML → CSS → Layout → Render pipeline
- ✓ Live demo application
- ✓ Complete architecture working

---

## 🎉 Phase 2 Complete!

The browser can now **parse HTML/CSS and render it to screen** with GPU acceleration!

**What Works:**
- Parse HTML documents
- Apply CSS styles with cascade/specificity
- Calculate layouts with box model
- Render backgrounds and borders
- Window management and events

**Next: Phase 4**
- Address bar and navigation UI
- JavaScript engine integration
- More CSS features (flexbox, grid)
- Font rendering completion
- Image decoding and rendering

## Architecture

The browser follows a traditional rendering pipeline:

```
HTTP → HTML → DOM Tree → Style Tree → Layout Tree → Display List → Pixels
```

**Current Implementation:**
- HTTP → HTML ✓ (Phase 3)
- HTML → DOM Tree ✓
- DOM + CSS → Style Tree ✓
- Style → Layout Tree ✓
- Layout → Display List ✓ (Phase 2)
- Display List → Pixels ✓ (Phase 2)

## License

This is an educational project for learning browser engine architecture.
