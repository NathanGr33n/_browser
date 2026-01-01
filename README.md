# Browser Engine

A high-performance, educational browser engine built from scratch in Rust, featuring a complete rendering pipeline from HTML to GPU-accelerated pixels.

[![Tests](https://img.shields.io/badge/tests-92%20passing-brightgreen)]() [![Rust](https://img.shields.io/badge/rust-edition%202021-orange)]() [![License](https://img.shields.io/badge/license-Educational-blue)]()

## Overview

This browser engine implements the core components of a modern web browser:
- **HTML5 parsing** with standards-compliant DOM construction
- **CSS parsing** and style computation with cascade and specificity
- **Layout engine** featuring CSS box model and flexbox
- **GPU-accelerated rendering** using WebGPU (wgpu)
- **Networking layer** with HTTP client and resource caching
- **Browser UI** with address bar and navigation controls
- **JavaScript runtime** with DOM bindings and event system
- **Font rendering** with system font support and glyph caching
- **Image decoding** for PNG, JPEG, GIF, and WebP formats

## Quick Start

```bash
# Run the main demo
cargo run

# Run all tests
cargo test --lib

# Run network demo (fetch real websites)
cargo run --example network_demo
```

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

## Features

### Parsing & DOM
- HTML5 parsing with html5ever
- DOM tree construction with elements, text nodes, and attributes
- CSS parsing with selectors (tag, class, ID, combinators)
- Style computation with cascade, specificity, and inheritance

### Layout Engine
- CSS box model implementation (content, padding, border, margin)
- Block and inline layout
- Flexbox layout foundation (direction, wrap, justify-content, align-items)
- Width/height calculations with auto values
- Position calculation and box tree generation

### Rendering
- GPU-accelerated rendering via wgpu (WebGPU)
- Rectangle rendering with alpha blending
- Border rendering (per-edge width and color)
- Display list generation and optimization
- Viewport culling for performance
- Scrolling infrastructure

### Networking
- HTTP/HTTPS client with blocking and async APIs
- Resource loading with 50MB LRU cache
- CSS extraction from `<style>` tags and `<link>` elements
- Page loading with integrated parsing

### Browser UI
- Address bar with URL input and editing
- Navigation controls (back, forward, refresh)
- Loading progress indicators
- Navigation history management
- Mouse and keyboard input handling

### JavaScript
- Basic JavaScript runtime (foundation for V8/SpiderMonkey)
- Value types (undefined, null, boolean, number, string, object, array, function)
- DOM bindings for element access
- Event system (click, keyboard, mouse, load, scroll)
- Console logging
- Execution security controls

### Media Support
- Font manager with system font loading
- Font caching and fallback system
- Glyph rasterization with texture atlas
- Text measurement API
- Image decoding (PNG, JPEG, GIF, WebP)
- Image cache with LRU eviction (100MB default)
- Automatic format detection

## Usage Examples

### Running Demos

```bash
# Main demo application
cargo run

# Network demo - fetches and renders real websites
cargo run --example network_demo

# Full browser demo - complete HTML+CSS rendering
cargo run --bin browser_demo
```

### Component Tests

```bash
# Run all unit tests (92 tests)
cargo test --lib

# Run specific component tests
cargo run --bin window_test   # Window creation and GPU init
cargo run --bin rect_test     # Rectangle rendering
cargo run --bin border_test   # Border rendering
```

## Project Structure

```
src/
├── dom/          # DOM tree representation
├── html/         # HTML parser (html5ever integration)
├── css/          # CSS parser and value types
├── style/        # Style computation and selector matching
├── layout/       # Layout engine with box model and flexbox
├── display/      # Display list generation
├── window/       # Window management
├── renderer/     # GPU renderer with wgpu, fonts, and images
├── net/          # HTTP client and resource loading
├── ui/           # Browser UI (address bar, navigation)
├── js/           # JavaScript engine integration
├── lib.rs        # Library interface
├── main.rs       # Demo application
└── bin/          # Test binaries
```

## Technology Stack

**Core Technologies:**
- **Language**: Rust (edition 2021)
- **HTML Parser**: html5ever (W3C-compliant)
- **CSS Parser**: cssparser
- **Graphics**: wgpu (WebGPU API)
- **Window Management**: winit (cross-platform)
- **Networking**: reqwest with tokio async runtime
- **Font Rendering**: fontdue + font-kit
- **Image Decoding**: image crate (PNG, JPEG, GIF, WebP)
- **URL Parsing**: url crate

### Phase 4: Advanced Features ✓ COMPLETE

**Browser UI & Navigation**

- ✓ Address bar with URL input and editing
- ✓ Navigation buttons (back, forward, refresh)
- ✓ Loading progress indicators
- ✓ Browser chrome and content viewport separation
- ✓ Input handling (mouse and keyboard)

**JavaScript Engine Integration**

- ✓ Basic JavaScript runtime (stub for V8/SpiderMonkey integration)
- ✓ DOM bindings for JavaScript access
- ✓ Event handling system (click, keyboard, etc.)
- ✓ JavaScript context with execution control
- ✓ Console logging support

**Enhanced CSS Features**

- ✓ Flexbox layout foundation (direction, wrap, justify, align)
- ✓ Flex container and flex item properties
- ✓ CSS property parsing for flexbox

**Font Rendering**

- ✓ Font manager with system font loading
- ✓ Font caching by family name
- ✓ Text measurement API
- ✓ Glyph cache with texture atlas
- ✓ Font fallback system

**Image Support**

- ✓ Image decoding (PNG, JPEG, GIF, WebP)
- ✓ Image cache with LRU eviction
- ✓ RGBA8 texture format conversion
- ✓ Automatic format detection

---

## 🎉 Phase 4 Complete!

The browser now includes:
- **Full UI**: Address bar, navigation buttons, and user input handling
- **JavaScript Integration**: Basic runtime with DOM bindings and event system
- **Advanced CSS**: Flexbox layout foundation
- **Font Rendering**: System font loading with caching and measurement
- **Image Support**: Multi-format decoding with intelligent caching

**Test Coverage**: 92 unit tests passing

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

## Development

### Prerequisites

- Rust 1.70+ (edition 2021)
- System font libraries (automatically detected by font-kit)
- GPU with WebGPU support

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Check code without building
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Generate documentation
cargo doc --open
```

## Architecture Notes

### Rendering Pipeline

The engine follows a traditional browser rendering pipeline:

```
HTTP Request → HTML → DOM Tree → Style Tree → Layout Tree → Display List → GPU Rendering
```

### Module Organization

- **dom/**: Core DOM data structures
- **html/**: HTML5 parsing
- **css/**: CSS parsing and value types
- **style/**: Style computation and selector matching
- **layout/**: Layout engine (box model, flexbox)
- **display/**: Display list generation
- **renderer/**: GPU rendering, fonts, images
- **window/**: Window and event management
- **net/**: HTTP client and caching
- **ui/**: Browser UI components
- **js/**: JavaScript runtime integration

### Design Decisions

- **Blocking network I/O**: Simplifies the architecture for educational purposes
- **Basic JavaScript runtime**: Stub implementation showing how a real engine would integrate
- **GPU rendering**: Modern approach using WebGPU for cross-platform compatibility
- **No unsafe code in core modules**: Prioritizes safety and clarity

## Future Enhancements

Potential areas for expansion:

- Full V8 or SpiderMonkey JavaScript engine integration
- CSS Grid layout implementation
- Additional CSS selectors (pseudo-classes, attribute selectors)
- WebSocket support
- Local storage and cookies
- Form handling and input elements
- SVG rendering
- Web Workers
- DevTools integration

## Contributing

This is an educational project. Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass (`cargo test`)
5. Format code (`cargo fmt`)
6. Submit a pull request

## License

This is an educational project for learning browser engine architecture.

## Acknowledgments

- [Let's build a browser engine!](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html) by Matt Brubeck
- [Servo](https://servo.org/) - Mozilla's experimental browser engine
- The Rust community for excellent libraries

## Resources

- [HTML5 Specification](https://html.spec.whatwg.org/)
- [CSS Specification](https://www.w3.org/Style/CSS/)
- [WebGPU Specification](https://gpuweb.github.io/gpuweb/)
- [Browser Engine Architecture](https://www.html5rocks.com/en/tutorials/internals/howbrowserswork/)
