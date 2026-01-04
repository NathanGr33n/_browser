# Browser Engine

A high-performance, educational browser engine built from scratch in Rust, featuring a complete rendering pipeline from HTML to GPU-accelerated pixels.

[![Tests](https://img.shields.io/badge/tests-277%20passing-brightgreen)]() [![Rust](https://img.shields.io/badge/rust-edition%202021-orange)]() [![License](https://img.shields.io/badge/license-Educational-blue)]()

## Overview

This browser engine implements the core components of a modern web browser:
- **HTML5 parsing** with standards-compliant DOM construction
- **CSS parsing** and style computation with cascade and specificity
- **Layout engine** featuring CSS box model and flexbox
- **GPU-accelerated rendering** using WebGPU (wgpu)
- **Networking layer** with HTTP client and resource caching
- **Browser UI** with address bar and navigation controls
- **JavaScript runtime** with Boa engine integration and DOM bindings
- **Font rendering** with system font support and glyph caching
- **Image decoding** for PNG, JPEG, GIF, and WebP formats
- **CSS Grid Layout** with track sizing and item placement
- **Form handling** with input, textarea, button, and select elements
- **Developer tools** with console, DOM inspector, and network tab
- **CSS Animations** with keyframes and transitions
- **Canvas 2D API** with path rendering and image drawing
- **Web Storage** with LocalStorage, SessionStorage, and Cookies
- **WebSocket protocol** with real-time bidirectional communication
- **Multi-process architecture** with per-tab renderer isolation
- **DOM Observers** with MutationObserver, IntersectionObserver, and ResizeObserver
- **Performance APIs** with timing, marks, measures, and resource tracking
- **Fetch API** with Request/Response objects and CORS handling
- **IndexedDB** with object stores, indexes, cursors, and transactions

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
├── dom/            # DOM tree representation
├── html/           # HTML parser (html5ever integration)
├── css/            # CSS parser and value types
├── style/          # Style computation and selector matching
├── layout/         # Layout engine with box model, flexbox, and grid
├── display/        # Display list generation
├── window/         # Window management
├── renderer/       # GPU renderer with wgpu, fonts, and images
├── net/            # HTTP client and resource loading
├── ui/             # Browser UI (address bar, navigation)
├── js/             # Boa JavaScript engine integration
├── navigation/     # Navigation history management
├── forms/          # Form handling (input, textarea, select)
├── devtools/       # Developer tools (console, DOM inspector, network)
├── compositor/     # Layer-based compositor with tile rendering
├── animation/      # CSS animations and transitions
├── canvas/         # Canvas 2D API implementation
├── storage/        # LocalStorage, SessionStorage, Cookies
├── websocket/      # WebSocket protocol (RFC 6455)
├── multiprocess/   # Multi-process architecture with IPC
├── observers/      # DOM Observers (Mutation, Intersection, Resize)
├── performance/    # Performance timing and monitoring APIs
├── fetch/          # Fetch API with CORS and streaming
├── indexeddb/      # IndexedDB client-side database
├── benchmarks/     # Boa JavaScript engine benchmarks
├── lib.rs          # Library interface
├── main.rs         # Demo application
└── bin/            # Test binaries
```

## Technology Stack

**Core Technologies:**
- **Language**: Rust (edition 2021)
- **HTML Parser**: html5ever (W3C-compliant)
- **CSS Parser**: cssparser
- **JavaScript**: Boa (ECMAScript engine)
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

### Phase 5: Advanced Layout ✓ COMPLETE

**CSS Grid Layout**

- ✓ Grid container with template rows/columns
- ✓ Grid item placement (auto and explicit)
- ✓ Track sizing with fr units, auto, and fixed sizes
- ✓ Gap properties (row-gap, column-gap)
- ✓ 13 comprehensive Grid tests

### Phase 6: Interactive Browser ✓ COMPLETE

**Navigation & History**

- ✓ Navigation history with back/forward
- ✓ History state management
- ✓ URL tracking and navigation

**Form Handling**

- ✓ Input fields (text, password, email, number, date)
- ✓ Textarea with multiline text support
- ✓ Buttons (submit, reset, button)
- ✓ Select dropdowns with options
- ✓ Form validation and state management

**JavaScript Engine**

- ✓ Boa JavaScript engine integration
- ✓ ECMAScript execution
- ✓ DOM manipulation from JS
- ✓ Event handlers and callbacks

**Developer Tools**

- ✓ Console with log/warn/error output
- ✓ DOM inspector with tree view
- ✓ Network tab with request/response tracking
- ✓ DevTools panel UI

### Phase 7: Modern Web Features ✓ COMPLETE

**CSS Animations & Transitions**

- ✓ Keyframe animations with @keyframes
- ✓ Timing functions (linear, ease, cubic-bezier, steps)
- ✓ CSS transitions with delays
- ✓ Value interpolation for smooth animations

**Canvas 2D API**

- ✓ Full 2D drawing context
- ✓ Path rendering (lines, curves, arcs)
- ✓ Rectangle and shape operations
- ✓ Text rendering on canvas
- ✓ Image drawing with scaling
- ✓ Alpha blending and compositing

**Web Storage APIs**

- ✓ LocalStorage with persistent storage (5MB quota)
- ✓ SessionStorage for per-tab storage
- ✓ Cookie management with attributes
- ✓ Storage events for cross-window sync
- ✓ Quota enforcement

**WebSocket Protocol**

- ✓ RFC 6455 compliant implementation
- ✓ Connection state management
- ✓ Text and binary message framing
- ✓ Ping/pong heartbeat mechanism
- ✓ Secure WebSocket (wss://) support

**Multi-Process Architecture**

- ✓ Per-tab renderer process isolation (up to 100)
- ✓ IPC message queue system (1000 message limit)
- ✓ Shared memory for rendering (100MB limit)
- ✓ Crash isolation and cleanup
- ✓ Process lifecycle management

---

## 🎉 Phase 7 Complete!

The browser now includes a comprehensive set of modern web features:
- **54 new tests added** in Phase 7 alone
- **6 new modules**: compositor, animation, canvas, storage, websocket, multiprocess
- **3,667 lines** of production code
- **220 total tests** passing

### Phase 8: Advanced JavaScript ✓ COMPLETE

**DOM Observers**

- ✓ MutationObserver for DOM change detection (childList, attributes, characterData, subtree)
- ✓ IntersectionObserver for viewport intersection tracking with thresholds
- ✓ ResizeObserver for element size change monitoring
- ✓ Mutation record batching and observer management

**Performance APIs**

- ✓ High-resolution timing with performance.now()
- ✓ User Timing API (marks and measures)
- ✓ Navigation Timing API (21 timing points: DNS, TCP, TLS, DOM events)
- ✓ Resource Timing API with 150-entry buffer and comprehensive metrics
- ✓ Memory Info API for JS heap tracking

**Fetch API**

- ✓ Complete async fetch() implementation with RequestInfo enum
- ✓ Request object (method, headers, body, mode, credentials, cache, redirect)
- ✓ Response object (status, headers, text/json/bytes consumption)
- ✓ Headers API with case-insensitive operations
- ✓ CORS modes (SameOrigin, Cors, NoCors, Navigate)
- ✓ Redirect handling with status validation (301/302/303/307/308)

**IndexedDB**

- ✓ IDBFactory for database creation/deletion
- ✓ IDBDatabase with object store management and versioning
- ✓ IDBObjectStore with CRUD operations (add, put, get, delete, clear, count)
- ✓ IDBIndex for secondary key queries with unique/multiEntry support
- ✓ IDBCursor for record iteration (Next, Prev, NextUnique, PrevUnique)
- ✓ IDBKeyRange for range queries (only, bound, lowerBound, upperBound)
- ✓ IDBTransaction with ReadOnly, ReadWrite, VersionChange modes
- ✓ Auto-increment keys and key path support
- ✓ Serialization with serde for persistence

**JavaScript Performance Benchmarking**

- ✓ Comprehensive Boa engine benchmark suite (8 categories)
- ✓ Performance analysis showing 361.97 avg ops/sec
- ✓ Benchmark runner example for performance testing

---

## 🎉 Phase 8 Complete!

The browser now supports advanced JavaScript APIs for rich web applications:
- **57 new tests added** in Phase 8
- **5 new modules**: observers, performance, fetch, indexeddb, benchmarks
- **2,603 lines** of production code  
- **277 total tests** passing (up from 220)

**Benchmark Results** (release mode, 100 iterations):
- Array Operations: 92.92 ops/sec
- Object Operations: 493.72 ops/sec
- Function Calls: 147.62 ops/sec
- String Operations: 1,145.04 ops/sec
- DOM-like Operations: 169.75 ops/sec
- TodoMVC Pattern: 234.25 ops/sec
- JSON Operations: 263.24 ops/sec
- Class Patterns: 349.21 ops/sec

**Test Coverage**: 277 unit tests passing

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
- **layout/**: Layout engine (box model, flexbox, grid)
- **display/**: Display list generation
- **renderer/**: GPU rendering, fonts, images
- **window/**: Window and event management
- **net/**: HTTP client and caching
- **ui/**: Browser UI components
- **js/**: Boa JavaScript engine integration
- **navigation/**: History and navigation management
- **forms/**: Form handling and validation
- **devtools/**: Developer tools and debugging
- **compositor/**: Layer-based rendering with tiling
- **animation/**: CSS animations and transitions
- **canvas/**: Canvas 2D API
- **storage/**: Web Storage and Cookies
- **websocket/**: WebSocket protocol
- **multiprocess/**: Process isolation and IPC

### Design Decisions

- **Blocking network I/O**: Simplifies the architecture for educational purposes
- **Basic JavaScript runtime**: Stub implementation showing how a real engine would integrate
- **GPU rendering**: Modern approach using WebGPU for cross-platform compatibility
- **No unsafe code in core modules**: Prioritizes safety and clarity

## Future Enhancements

Potential areas for expansion:

- SVG rendering
- Web Workers and Service Workers
- WebRTC support
- Audio/Video playback
- WebAssembly integration
- Additional CSS selectors (pseudo-classes, attribute selectors)
- CSS animations and transitions refinement
- IndexedDB implementation
- Content Security Policy
- CORS handling

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
