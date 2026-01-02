# Phase 5: Complete Rendering - Implementation Plan

**Duration:** 3 months (12 weeks)  
**Goal:** Render static websites perfectly  
**Success Criteria:** Wikipedia homepage renders correctly with text, images, and layouts

---

## Week 1-3: Complete Flexbox Layout

### Overview
Implement the full CSS Flexbox specification algorithm. Currently, only property parsing exists - need the actual layout logic.

### Tasks

#### Week 1: Flexbox Core Algorithm
**Goal:** Basic flex container layout working

**Implementation Steps:**

1. **Main Axis Calculation** (2-3 days)
   - [ ] Determine main/cross axis based on flex-direction
   - [ ] Calculate available space in main axis
   - [ ] Resolve flex-basis for all items
   - [ ] Calculate hypothetical main size for each item

2. **Flex Growing/Shrinking** (2-3 days)
   - [ ] Calculate total flex-grow/shrink factors
   - [ ] Distribute extra space (flex-grow)
   - [ ] Shrink items if overflow (flex-shrink)
   - [ ] Handle min/max-width constraints

3. **Main Axis Positioning** (1-2 days)
   - [ ] Implement justify-content
   - [ ] Position items along main axis
   - [ ] Handle flex-start, flex-end, center
   - [ ] Space-between, space-around, space-evenly

#### Week 2: Cross Axis & Wrapping
**Goal:** Multi-line flex containers work

1. **Cross Axis Sizing** (2 days)
   - [ ] Calculate cross size for each item
   - [ ] Implement align-items (stretch, flex-start, flex-end, center, baseline)
   - [ ] Handle align-self on flex items

2. **Flex Wrap** (2-3 days)
   - [ ] Implement line breaking algorithm
   - [ ] Handle wrap and wrap-reverse
   - [ ] Calculate line sizes
   - [ ] Distribute items across lines

3. **Align Content** (1-2 days)
   - [ ] Align flex lines in container
   - [ ] Implement all align-content values

#### Week 3: Testing & Edge Cases
**Goal:** Robust, production-ready flexbox

1. **Edge Cases** (2 days)
   - [ ] Zero-sized flex containers
   - [ ] Single item containers
   - [ ] Deeply nested flex containers
   - [ ] Mix of flex and block children

2. **Testing** (2-3 days)
   - [ ] Create comprehensive flexbox test suite
   - [ ] Test against W3C flexbox tests
   - [ ] Real-world layout tests (nav bars, cards)
   - [ ] Performance benchmarks

3. **Integration** (1-2 days)
   - [ ] Wire flexbox into main layout engine
   - [ ] Handle display: flex vs display: block
   - [ ] Update layout_tree to detect flex containers

**Files to Modify:**
- `src/layout/flexbox.rs` - Main implementation
- `src/layout/mod.rs` - Integration with layout engine
- `src/layout/flexbox_tests.rs` - Comprehensive tests (new file)

**Key Algorithm Reference:**
- [CSS Flexbox Spec](https://www.w3.org/TR/css-flexbox-1/#layout-algorithm)
- Servo's flexbox implementation (study for reference)

---

## Week 4-7: Implement CSS Grid

### Overview
Implement CSS Grid Layout - more complex than flexbox but essential for modern layouts.

### Tasks

#### Week 4: Grid Fundamentals
**Goal:** Basic grid container working

1. **Grid Track Definition** (3 days)
   - [ ] Parse grid-template-columns/rows
   - [ ] Handle fixed sizes (px, %)
   - [ ] Handle fr units
   - [ ] Handle repeat() notation

2. **Grid Item Placement** (3-4 days)
   - [ ] Parse grid-column/row properties
   - [ ] Implement auto-placement algorithm
   - [ ] Handle grid-area
   - [ ] Explicit vs implicit grids

#### Week 5: Advanced Grid Features
**Goal:** Complex grids work

1. **Track Sizing Algorithm** (3-4 days)
   - [ ] Implement intrinsic sizing
   - [ ] Handle min-content, max-content
   - [ ] Implement minmax()
   - [ ] Auto track sizing

2. **Grid Gaps** (1 day)
   - [ ] Implement gap/row-gap/column-gap
   - [ ] Calculate with gaps in layout

3. **Named Grid Lines & Areas** (2 days)
   - [ ] Parse named lines
   - [ ] Parse grid-template-areas
   - [ ] Resolve names to positions

#### Week 6-7: Grid Polish & Testing
**Goal:** Production-ready grid

1. **Alignment** (2-3 days)
   - [ ] justify-items, align-items
   - [ ] justify-content, align-content
   - [ ] justify-self, align-self

2. **Dense Packing** (1-2 days)
   - [ ] Implement grid-auto-flow: dense

3. **Testing** (3-4 days)
   - [ ] W3C Grid test suite
   - [ ] Real-world grid layouts
   - [ ] Performance benchmarks
   - [ ] Edge cases (empty grids, overflow)

4. **Integration** (1-2 days)
   - [ ] Wire into layout engine
   - [ ] Handle display: grid

**Files to Create/Modify:**
- `src/layout/grid.rs` - Grid implementation (new)
- `src/layout/mod.rs` - Integration
- `src/layout/grid_tests.rs` - Tests (new)

**Key Algorithm Reference:**
- [CSS Grid Spec](https://www.w3.org/TR/css-grid-2/)
- Detailed Grid algorithm (MDN docs)

---

## Week 8: CSS Positioning

### Overview
Implement position: absolute, relative, fixed, sticky - essential for modern layouts.

### Tasks

#### Week 8: All Position Types
**Goal:** All position values work correctly

1. **Position Property Parsing** (1 day)
   - [ ] Ensure position property is parsed
   - [ ] Parse offset properties (top, right, bottom, left)

2. **Relative Positioning** (1 day)
   - [ ] Offset from normal position
   - [ ] Doesn't affect other elements' layout

3. **Absolute Positioning** (2 days)
   - [ ] Find positioning context (nearest positioned ancestor)
   - [ ] Position relative to containing block
   - [ ] Remove from normal flow

4. **Fixed Positioning** (1 day)
   - [ ] Position relative to viewport
   - [ ] Doesn't scroll with page

5. **Sticky Positioning** (1-2 days)
   - [ ] Hybrid relative/fixed behavior
   - [ ] Track scroll position
   - [ ] Switch between modes

6. **Z-index & Stacking Contexts** (1-2 days)
   - [ ] Implement stacking context rules
   - [ ] Sort elements by z-index
   - [ ] Handle stacking without z-index

**Files to Modify:**
- `src/layout/mod.rs` - Add positioning logic
- `src/layout/positioned.rs` - Positioning module (new)
- `src/display/mod.rs` - Update for stacking contexts

---

## Week 9-10: Text Rendering

### Overview
Actually render text to the screen - critical blocker for usability.

### Tasks

#### Week 9: SDF Text Rendering
**Goal:** Text appears on screen

1. **SDF Shader** (2-3 days)
   - [ ] Write WGSL shader for SDF text
   - [ ] Implement distance field rendering
   - [ ] Subpixel anti-aliasing
   - [ ] Color and opacity

2. **Glyph Rendering Pipeline** (2 days)
   - [ ] Wire up glyph cache to GPU
   - [ ] Upload SDF textures to GPU
   - [ ] Batch text rendering (one draw call per frame)

3. **Text in Display List** (1-2 days)
   - [ ] Add Text display command
   - [ ] Extract text nodes from layout
   - [ ] Position text based on layout

#### Week 10: Text Features
**Goal:** Production-quality text

1. **Font Fallback** (1-2 days)
   - [ ] Try fonts in order
   - [ ] Handle missing glyphs
   - [ ] Emoji support (color fonts)

2. **Text Measurement in Layout** (2 days)
   - [ ] Integrate text measurement into box model
   - [ ] Calculate line boxes
   - [ ] Text wrapping

3. **Text Properties** (2 days)
   - [ ] font-weight (bold)
   - [ ] font-style (italic)
   - [ ] text-decoration (underline)
   - [ ] text-align

4. **Line Breaking** (1-2 days)
   - [ ] Break at word boundaries
   - [ ] Handle long words (overflow-wrap)
   - [ ] Hyphenation (basic)

**Files to Modify:**
- `src/renderer/shaders/text.wgsl` - SDF shader (new)
- `src/renderer/text_renderer.rs` - Complete implementation
- `src/display/mod.rs` - Add Text command
- `src/layout/mod.rs` - Text layout integration

**Dependencies:**
- Already have: fontdue, font-kit, glyph cache
- Need: HarfBuzz (for complex text shaping) - add later

---

## Week 11: Image Rendering

### Overview
Display images from `<img>` tags and CSS backgrounds.

### Tasks

#### Week 11: GPU Image Rendering
**Goal:** Images display correctly

1. **Image Shader** (1 day)
   - [ ] Write WGSL shader for textured quads
   - [ ] Handle aspect ratio
   - [ ] Alpha blending

2. **Image Upload to GPU** (1-2 days)
   - [ ] Convert decoded images to GPU textures
   - [ ] Cache textures
   - [ ] Handle image changes

3. **`<img>` Tag Support** (1 day)
   - [ ] Parse `<img>` elements
   - [ ] Load image from src attribute
   - [ ] Create layout box for image

4. **CSS Background Images** (1-2 days)
   - [ ] Parse background-image
   - [ ] Load images
   - [ ] Render as box background

5. **Image in Display List** (1 day)
   - [ ] Add Image display command
   - [ ] Position and size images
   - [ ] Handle image sizing (contain, cover)

**Files to Modify:**
- `src/renderer/shaders/image.wgsl` - Image shader (new)
- `src/renderer/image_renderer.rs` - Image rendering (new)
- `src/display/mod.rs` - Add Image command
- `src/dom/mod.rs` - Handle `<img>` elements
- Already have: image cache, decoding

---

## Week 12: Testing & Validation

### Overview
Comprehensive testing to ensure Phase 5 is production-ready.

### Tasks

#### Week 12: End-to-End Testing
**Goal:** Wikipedia renders correctly

1. **Layout Tests** (2 days)
   - [ ] Test complex real-world layouts
   - [ ] Flexbox + Grid combinations
   - [ ] Positioned elements over flex/grid
   - [ ] Nested layouts

2. **Rendering Tests** (2 days)
   - [ ] Test text rendering quality
   - [ ] Verify images load and display
   - [ ] Check colors, fonts, sizes
   - [ ] Screenshot comparison tests

3. **Performance Benchmarks** (1 day)
   - [ ] Layout performance (complex pages)
   - [ ] Render performance (frame times)
   - [ ] Memory usage
   - [ ] Cold start time

4. **Wikipedia Rendering** (2-3 days)
   - [ ] Load Wikipedia homepage
   - [ ] Fix any layout issues
   - [ ] Ensure text is readable
   - [ ] Images load correctly
   - [ ] Compare to Chrome/Firefox

5. **Bug Fixes** (remaining time)
   - [ ] Fix critical bugs found
   - [ ] Optimize hot paths
   - [ ] Polish edge cases

**Success Criteria:**
✅ Wikipedia homepage renders correctly (90%+ accurate)  
✅ All text visible and readable  
✅ Images display properly  
✅ Layouts match real browsers (flexbox/grid work)  
✅ Performance: <16ms frame time for Wikipedia  
✅ 150+ tests passing (up from 92)

---

## Implementation Order Summary

```
Week 1-3:  Complete Flexbox
Week 4-7:  Implement CSS Grid  
Week 8:    CSS Positioning
Week 9-10: Text Rendering (SDF)
Week 11:   Image Rendering
Week 12:   Testing & Validation
```

## Critical Path

The **critical path** (must be done in order):
1. **Flexbox** → Enables modern layouts
2. **Grid** → Completes layout engine
3. **Positioning** → Enables complex layouts
4. **Text** → Makes content visible (CRITICAL)
5. **Images** → Completes media rendering
6. **Testing** → Ensures quality

**Text Rendering is the critical blocker** - without it, you can't see most content even if layout is perfect.

## Parallelization Opportunities

If working on multiple features concurrently:
- **Flexbox & Positioning** can be developed in parallel (different code paths)
- **Text & Image rendering** share similar GPU patterns (can reuse shader techniques)
- **Testing** should be ongoing throughout, not just Week 12

---

## Risk Mitigation

### High-Risk Areas

1. **Flexbox Algorithm Complexity**
   - Risk: Algorithm is intricate, easy to get wrong
   - Mitigation: Study Servo's impl, test incrementally

2. **Text Rendering Quality**
   - Risk: Bad text rendering = unusable browser
   - Mitigation: Use proven techniques (SDF), test on many fonts

3. **Performance**
   - Risk: Slow layout = bad UX
   - Mitigation: Profile early, optimize hot paths, benchmark continuously

### Contingency Plans

- If **Flexbox takes >3 weeks**: Consider implementing subset first (no wrapping, simpler alignment)
- If **Grid takes >4 weeks**: Defer advanced features (named areas, dense packing)
- If **Text rendering takes >2 weeks**: Start with bitmap glyphs (simpler), add SDF later
- If **Behind schedule overall**: Defer image rendering to Phase 6, prioritize text

---

## Daily Development Workflow

**Suggested routine:**

1. **Morning (2-3 hours):**
   - Implement one feature/function
   - Write tests for that feature
   - Commit when tests pass

2. **Afternoon (2-3 hours):**
   - Continue implementation
   - Debug issues from morning
   - Update documentation

3. **End of Day:**
   - Run full test suite
   - Commit working code
   - Plan next day's tasks

**Weekly:**
- Friday: Review week's progress vs plan
- Saturday/Sunday: Learning/research for next week

**Avoid:**
- Working on multiple features at once (finish one first)
- Committing broken code
- Skipping tests (they'll catch bugs early)

---

## Tools & Resources

### Development Tools
- **VS Code** with rust-analyzer
- **cargo watch** - auto-recompile on save
- **cargo flamegraph** - performance profiling
- **GPU profiler** (RenderDoc or similar)

### Testing Tools
- **cargo test** - unit tests
- **cargo bench** - benchmarks
- **Screenshot comparison** - against Chrome

### References
- [Flexbox Spec](https://www.w3.org/TR/css-flexbox-1/)
- [Grid Spec](https://www.w3.org/TR/css-grid-2/)
- [Servo Layout Code](https://github.com/servo/servo/tree/main/components/layout_2020)
- [MDN CSS Docs](https://developer.mozilla.org/en-US/docs/Web/CSS)

---

## Next Steps

**Immediate Action Items:**

1. [ ] Review this plan, adjust if needed
2. [ ] Set up development environment
3. [ ] Create feature branch: `phase-5-flexbox`
4. [ ] Start implementing flexbox main axis calculation
5. [ ] Write first test: simple flex container with 3 items

**Ready to start?** Let's begin with Flexbox Week 1! 🚀
