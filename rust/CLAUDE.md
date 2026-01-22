# Alexandria Landing Page (Pure Rust Canvas) - Claude Guide

## IMPORTANT: LOCAL DEVELOPMENT ONLY

**DO NOT deploy this Rust version to mainnet.** The production frontend is the TypeScript version at `/home/theseus/alexandria/alex_landing_page/typescript/`. This Rust version is experimental and for local development only.

To deploy the production frontend:
```bash
cd /home/theseus/alexandria/alex_landing_page/typescript && dfx deploy --network ic alex_landing_page_frontend
```

---

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

## Production Canister (TypeScript version)

| Canister ID | URL |
|-------------|-----|
| `z6d57-uyaaa-aaaau-ac24a-cai` | https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io |

**Note:** This URL runs the TypeScript frontend, not this Rust version.

## Commands

```bash
# Install Trunk (one-time)
cargo install trunk

# Local development
trunk serve --port 8080

# Build
trunk build --release

# DO NOT deploy to mainnet - use TypeScript version instead
# See: /home/theseus/alexandria/alex_landing_page/typescript/
```

## Autonomous Debugging (IMPORTANT)

**NEVER ask the user to check the browser or copy/paste logs.** Use Playwright to capture everything automatically.

### Quick Debug Command

```bash
cd /home/theseus/alexandria/alex_landing_page/rust/testing && npm run capture:prod
```

Then read these files:
1. `cat preview_output/summary.json` - Check `success`, `errorCount`, `errors[]`
2. `cat preview_output/console.log` - Full console output with `[RUST]` debug logs
3. View `preview_output/screenshot_desktop.png` - See what the page looks like

### Full Agent Workflow

```bash
# 1. Make code changes to src/lib.rs

# 2. Build WASM
cd /home/theseus/alexandria/alex_landing_page/rust && trunk build

# 3. Deploy to production (if testing deployed version)
dfx deploy --network ic alex_landing_page_frontend

# 4. Capture screenshots and logs
cd testing && npm run capture:prod

# 5. Check results
cat preview_output/summary.json   # Quick status
cat preview_output/console.log    # Full logs if errors

# 6. View screenshots to verify rendering
# (Use Read tool on preview_output/screenshot_desktop.png)

# 7. If errors, fix and repeat from step 1
```

### Output Files

```
testing/preview_output/
├── screenshot_desktop.png   # 1920x1080 - main view
├── screenshot_mobile.png    # 390x844 - responsive check
├── screenshot_tablet.png    # 1024x768 - tablet view
├── console.log              # ALL browser console output
└── summary.json             # Machine-readable status
```

### Interpreting Results

**summary.json:**
```json
{
  "success": true,      // false if any errors
  "errorCount": 0,      // Number of JS/WASM errors
  "errors": [],         // Array of error messages
  "warnings": [...]     // Warnings (often ignorable)
}
```

**console.log patterns:**
- `[RUST] WASM module loaded` - WASM initialized OK
- `[RUST] Starting application` - App is starting
- `[RUST] Base URL for images:` - Image loading debug
- `RuntimeError: unreachable` - Rust panic (check last [RUST] log before this)
- `Failed to load resource` - Missing file or network error

### Local Development Testing

```bash
# Terminal 1: Start dev server
cd /home/theseus/alexandria/alex_landing_page/rust && trunk serve

# Terminal 2: Capture
cd testing && npm run capture:local
```

### One-time Setup (already done)

```bash
cd testing && npm install && npx playwright install chromium
```

## Structure

```
├── src/
│   ├── lib.rs              # WASM app: styles, layout, components
│   └── bin/
│       └── preview.rs      # Headless software renderer (optional)
├── testing/
│   ├── capture.ts          # Playwright capture script
│   ├── package.json        # npm scripts: capture, capture:local, capture:prod
│   └── preview_output/     # Screenshots and logs
├── index.html              # Minimal: just <canvas id="canvas">
├── public/favicon.ico      # Site icon
├── dist/                   # Built WASM + JS loader
├── Cargo.toml              # egui, eframe, wasm-bindgen
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

- **LOCAL DEVELOPMENT ONLY** - Do not deploy this to mainnet
- Production uses the TypeScript frontend at `../typescript/`
- Use `trunk serve` for local testing
- WASM binary is ~2.5MB unoptimized (wasm-opt disabled for compatibility)
