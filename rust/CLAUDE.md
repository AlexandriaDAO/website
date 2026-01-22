# Alexandria Landing Page (Rust/Leptos) - Claude Guide

## Overview

Pure Rust implementation of the Alexandria DAO landing page using Leptos, compiled to WebAssembly. An **experimental alternative** to the TypeScript version.

## Canister Info

| Canister ID | URL |
|-------------|-----|
| `z6d57-uyaaa-aaaau-ac24a-cai` | https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io |

**Note**: Deploys to the same canister as the TypeScript version.

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
```

## Structure

```
├── src/
│   └── lib.rs          # All components: TypewriterText, Hero, Metrics, Products, Footer
├── style.css           # CSS (dark theme, responsive, same design as TS version)
├── index.html          # Entry point + Trunk directives for assets
├── public/logos/       # Project & exchange logos
├── dist/               # Built WASM + assets -> deployed to IC
├── Cargo.toml          # leptos, wasm-bindgen, gloo-timers
├── Trunk.toml          # Build config
├── dfx.json            # Canister config
└── canister_ids.json   # z6d57-uyaaa-aaaau-ac24a-cai
```

## Key Files

- **lib.rs** - Single file with all components:
  - `TypewriterText` - Animated text cycling through 18 ICP tech keywords
  - `Hero` - Title + subtitle
  - `Metrics` - 4 stats (50K+ NFTs, 7 dApps, etc.)
  - `Products` - 7 products with logos, tags, descriptions
  - `Footer` - Social/exchange links with SVG icons
  - `main()` - Entry point with `#[wasm_bindgen(start)]`

- **style.css** - Pure CSS (converted from SCSS), same visual design as TypeScript version

- **index.html** - Contains Trunk directives:
  - `data-trunk rel="rust"` - Compile Rust to WASM
  - `data-trunk rel="css"` - Bundle CSS
  - `data-trunk rel="copy-dir"` - Copy logo assets

## Dependencies

```toml
leptos = { version = "0.6", features = ["csr"] }
wasm-bindgen = "0.2"
gloo-timers = { version = "0.3", features = ["futures"] }
console_error_panic_hook = "0.1"
```

## Important

- **Mainnet only** - No local replica, all deploys are LIVE
- Always run `trunk build --release` before `dfx deploy`
- The `#[wasm_bindgen(start)]` attribute on `main()` is required for auto-start
- WASM binary is ~300KB (release) / ~1.3MB (dev)
