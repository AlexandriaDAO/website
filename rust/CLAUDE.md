# Alexandria Landing Page (Pure Rust Canvas) - Claude Guide

## Overview

**ZERO CSS. ZERO DOM.** Pure Rust canvas-rendered landing page using egui/eframe, compiled to WebAssembly. A portfolio flex demonstrating that modern web UIs don't need CSS or the DOM - just a canvas and Rust.

## Architecture

This is NOT a typical web app. The entire UI renders to a single `<canvas>` element via WebGL:

- **No CSS files** - All styling defined as Rust structs (`ColorPalette`, `Typography`, `Layout`)
- **No DOM elements** - Only one `<canvas id="canvas">` exists
- **No React/Vue/Leptos** - Pure immediate-mode GUI with egui
- **All animations in Rust** - Typewriter effect, particle system, cursor blink, hover states

## Visual Features

- **Cyberpunk aesthetic** - Dark theme with purple/blue gradient accents
- **Procedural background** - Animated grid lines, floating particles, scanline effect
- **Gradient text rendering** - Per-character color interpolation
- **Responsive breakpoints** - Typography and layout adapt to screen width (all in Rust)
- **Hover effects** - Hit-testing and visual feedback without CSS `:hover`

## Canister Info

| Canister ID | URL |
|-------------|-----|
| `z6d57-uyaaa-aaaau-ac24a-cai` | https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io |

## Commands

```bash
# Install Trunk (one-time)
cargo install trunk

# Local development
trunk serve --port 8080

# Build for production
trunk build --release

# Deploy to mainnet (LIVE immediately)
dfx deploy --network ic alex_landing_page_frontend

# Check canister status
dfx canister --network ic status alex_landing_page_frontend

# Headless preview (generates PNG without browser)
cargo run --bin preview [width] [height]
```

## Headless Preview (Agent Visual Testing)

A software-rendered preview tool that generates PNG screenshots without requiring a browser or display. This enables AI agents to verify UI changes autonomously.

```bash
# Generate 1920x1080 preview (default)
cargo run --bin preview

# Custom resolution
cargo run --bin preview 1280 720

# Output location
preview_output/preview.png
```

**How it works:**
1. Creates a headless egui context
2. Runs UI rendering (same code as WASM version)
3. Tessellates shapes into triangles
4. Software-rasterizes to pixel buffer
5. Saves as PNG

**Use cases:**
- Verify rendering after code changes without opening a browser
- Autonomous visual regression testing
- Debug blank page issues (like the one this tool was created to solve)
- Generate screenshots for documentation

## Structure

```
├── src/
│   ├── lib.rs              # WASM app: styles, layout, components
│   └── bin/
│       └── preview.rs      # Headless software renderer
├── index.html              # Minimal: just <canvas id="canvas">
├── public/favicon.ico      # Site icon
├── dist/                   # Built WASM + JS loader
├── preview_output/         # Generated PNG screenshots
├── Cargo.toml              # egui, eframe, wasm-bindgen, image
├── Trunk.toml              # Build config
├── dfx.json                # Canister config
└── canister_ids.json       # z6d57-uyaaa-aaaau-ac24a-cai
```

## Key Components in lib.rs

### Style System (Rust Structs, No CSS)

```rust
struct ColorPalette {
    bg_primary: Color32,      // #000000
    accent_purple: Color32,   // #7c3aed
    accent_blue: Color32,     // #2563eb
    // ... all colors as Rust values
}

struct Typography {
    title_size: f32,          // Responsive font sizes
    subtitle_size: f32,
    // ...
}

struct Layout {
    max_width: f32,           // 1200px equivalent
    padding_x: f32,
    // ...
}
```

### Animation State

- `TypewriterState` - Cycles through 18 ICP keywords with type/delete animation
- `ParticleState` - 50 floating particles with position, velocity, alpha pulsing
- Cursor blink via timer

### Render Functions

- `render_background()` - Grid, particles, scanline, gradient glow
- `render_gradient_text()` - Per-character color interpolation
- `render_hero()` - Typewriter + subtitle
- `render_metrics()` - 4 stats with gradient values
- `render_product()` - Hit-testing, hover states, 3-column layout
- `render_footer_icon()` - Vector icons drawn with primitives

## Dependencies

```toml
eframe = "0.29"       # egui web framework
egui = "0.29"         # Immediate-mode GUI
egui_extras = "0.29"  # Image support
web-sys = "0.3"       # DOM access for canvas
wasm-bindgen = "0.2"  # Rust-JS interop
```

## Why This Exists

This is a demonstration that:
1. CSS is optional - all styling can live in Rust structs
2. The DOM is optional - canvas rendering works for full UIs
3. Immediate-mode GUI works for the web
4. You can build distinctive visuals without web frameworks

The cyberpunk aesthetic (grid, particles, scanlines) is intentionally impossible to achieve with standard CSS - these are procedural effects only possible with direct canvas access.

## Important

- **Mainnet only** - No local replica, all deploys are LIVE
- Always run `trunk build --release` before `dfx deploy`
- WASM binary is ~2.5MB unoptimized (wasm-opt disabled for compatibility)
- Continuous repaint requested for animations - this is intentional
