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
cargo test
```

## Project Structure

```
src/
├── dom/          # DOM tree representation
├── html/         # HTML parser (html5ever integration)
├── css/          # CSS parser and value types
├── style/        # Style computation and selector matching
├── layout/       # Layout engine with box model
└── main.rs       # Demo application
```

## Technology Stack

- **Language**: Rust (edition 2021)
- **HTML Parser**: html5ever
- **CSS Parser**: cssparser
- **Selector Matching**: selectors crate

## Next Steps: Phase 2

**Rendering Pipeline** (Months 3-4)

- Graphics backend (wgpu for GPU acceleration)
- Paint system for rendering boxes
- Text rendering
- Window management with winit
- Basic user interaction

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
